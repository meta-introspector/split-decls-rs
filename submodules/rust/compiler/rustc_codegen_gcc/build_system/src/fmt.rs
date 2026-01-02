mkuse!{use std :: ffi :: OsStr ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use crate :: utils :: { run_command_with_output , walk_dir } ;}

macro_rules! show_usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function show_usage in module {}", module_path!());
    };
}

mkfn!{
    show_usage_introspect!();
    fn show_usage () { println ! (r#"
`fmt` command help:

    --check                : Pass `--check` argument to `cargo fmt` commands
    --help                 : Show this help"#) ; }
}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    pub fn run () -> Result < () , String > { let mut check = false ; let args = std :: env :: args () . skip (2) ; for arg in args { match arg . as_str () { "--help" => { show_usage () ; return Ok (()) ; } "--check" => check = true , _ => return Err (format ! ("Unknown option {arg}")) , } } let cmd : & [& dyn AsRef < OsStr >] = if check { & [& "cargo" , & "fmt" , & "--check"] } else { & [& "cargo" , & "fmt"] } ; run_command_with_output (cmd , Some (Path :: new ("."))) ? ; run_command_with_output (cmd , Some (Path :: new ("build_system"))) ? ; run_rustfmt_recursively ("tests/run" , check) }
}

macro_rules! run_rustfmt_recursively_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_rustfmt_recursively in module {}", module_path!());
    };
}

mkfn!{
    run_rustfmt_recursively_introspect!();
    fn run_rustfmt_recursively < P > (dir : P , check : bool) -> Result < () , String > where P : AsRef < Path > , { walk_dir (dir , & mut | dir | run_rustfmt_recursively (dir , check) , & mut | file_path | { if file_path . extension () . filter (| ext | ext == & OsStr :: new ("rs")) . is_some () { let rustfmt_cmd : & [& dyn AsRef < OsStr >] = if check { & [& "rustfmt" , & "--check" , & file_path] } else { & [& "rustfmt" , & file_path] } ; run_command_with_output (rustfmt_cmd , Some (Path :: new ("."))) } else { Ok (()) } } , true ,) }
}