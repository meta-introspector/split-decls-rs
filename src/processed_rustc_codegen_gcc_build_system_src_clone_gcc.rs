// SRC: ../rust/compiler/rustc_codegen_gcc/build_system/src/clone_gcc.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::path::{Path, PathBuf};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

use crate::config::ConfigInfo;
use crate::utils::{git_clone, run_command_with_output};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=show_usage | COMPLEXITY=2 | LINES=11 */

fn show_usage() {
    println!(
        r#"
`clone-gcc` command help:

    --out-path         : Location where the GCC repository will be cloned (default: `./gcc`)"#
    );
    ConfigInfo::show_usage();
    println!("    --help                 : Show this help");
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=Args | COMPLEXITY=2 | LINES=6 */

#[derive(Default)]
struct Args {
    out_path: PathBuf,
    config_info: ConfigInfo,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=new | COMPLEXITY=29 | LINES=36 */

impl Args {
    fn new() -> Result<Option<Self>, String> {
        let mut command_args = Self::default();

        let mut out_path = None;

        // We skip binary name and the `clone-gcc` command.
        let mut args = std::env::args().skip(2);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--out-path" => match args.next() {
                    Some(path) if !path.is_empty() => out_path = Some(path),
                    _ => {
                        return Err("Expected an argument after `--out-path`, found nothing".into());
                    }
                },
                "--help" => {
                    show_usage();
                    return Ok(None);
                }
                arg => {
                    if !command_args.config_info.parse_argument(arg, &mut args)? {
                        return Err(format!("Unknown option {arg}"));
                    }
                }
            }
        }
        command_args.out_path = match out_path {
            Some(p) => p.into(),
            None => PathBuf::from("./gcc"),
        };
        Ok(Some(command_args))
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=run | COMPLEXITY=10 | LINES=22 */

pub fn run() -> Result<(), String> {
    let Some(args) = Args::new()? else {
        return Ok(());
    };

    let result = git_clone("https://github.com/rust-lang/gcc", Some(&args.out_path), false)?;
    if result.ran_clone {
        let gcc_commit = args.config_info.get_gcc_commit()?;
        println!("Checking out GCC commit `{gcc_commit}`...");
        run_command_with_output(
            &[&"git", &"checkout", &gcc_commit],
            Some(Path::new(&result.repo_dir)),
        )?;
    } else {
        println!(
            "There is already a GCC folder in `{}`, leaving things as is...",
            args.out_path.display()
        );
    }
    Ok(())
}