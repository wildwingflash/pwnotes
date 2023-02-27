use clap::Parser;
use std::fs;
use std::path::Path;

/// Simple program to create a folder structure to organize pwn
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[arg(short, long)]
    name: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    const FOLDERS: [&str; 10] = [
        "admin",
        "deliverables",
        "evidence/findings",
        "evidence/logging-output",
        "evidence/misc-files",
        "evidence/osint",
        "evidence/scans",
        "evidence/wireless",
        "evidence/notes",
        "retest",
    ];

    const FILENAMES: [&str; 9] = [
        "evidence/notes/1.admin-info.md",
        "evidence/notes/2.activity-log.md",
        "evidence/notes/3.payload-log.md",
        "evidence/notes/4.credentials.md",
        "evidence/notes/5.web-app-research.md",
        "evidence/notes/6.vuln-scan-research.md",
        "evidence/notes/7.service-enum-research.md",
        "evidence/notes/8.ad-enum-research.md",
        "evidence/notes/9.attack-path.md",
    ];

    let path = Path::new(&args.name);
    let path = match path.try_exists() {
        Ok(false) => path,
        _other => panic!("project name already exists: {}", &args.name),
    };

    for folder in FOLDERS {
        let new_path = path.join(folder);
        let _folder = match fs::create_dir_all(&new_path) {
            Err(why) => panic!("couldn't create {}:{}", new_path.display(), why),
            Ok(_folder) => println!("New folder: {}!", new_path.display()),
        };
    }

    for filename in FILENAMES {
        let new_path = path.join(filename);
        let _file = match fs::File::create(&new_path) {
            Err(why) => panic!("couldn't create {}:{}", new_path.display(), why),
            Ok(_file) => println!("New file: {}", new_path.display()),
        };
    }

    Ok(())
}
