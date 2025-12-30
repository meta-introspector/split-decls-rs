// Generated macro for run_cargo_command_with_callback (function)
macro_rules! Depcrate_testrun_cargo_command_with_callback {
() => {
// Module: crate::test
// Provides: {"run_cargo_command_with_callback"}
// Dependencies: {}
fn run_cargo_command_with_callback < F > (command : & [& dyn AsRef < OsStr >] , cwd : Option < & Path > , env : & Env , args : & TestArg , callback : F ,) -> Result < () , String > where F : Fn (& [& dyn AsRef < OsStr >] , Option < & Path > , & Env) -> Result < () , String > , { let toolchain = get_toolchain () ? ; let toolchain_arg = format ! ("+{toolchain}") ; let rustc_version = String :: from_utf8 (run_command_with_env (& [& args . config_info . rustc_command [0] , & "-V"] , cwd , Some (env)) ? . stdout ,) . map_err (| error | format ! ("Failed to retrieve rustc version: {error:?}")) ? ; let rustc_toolchain_version = String :: from_utf8 (run_command_with_env (& [& args . config_info . rustc_command [0] , & toolchain_arg , & "-V"] , cwd , Some (env) ,) ? . stdout ,) . map_err (| error | format ! ("Failed to retrieve rustc +toolchain version: {error:?}")) ? ; if rustc_version != rustc_toolchain_version { eprintln ! ("rustc_codegen_gcc is built for `{rustc_toolchain_version}` but the default rustc version is `{rustc_version}`." ,) ; eprintln ! ("Using `{rustc_toolchain_version}`.") ; } let mut env = env . clone () ; let rustflags = env . get ("RUSTFLAGS") . cloned () . unwrap_or_default () ; env . insert ("RUSTDOCFLAGS" . to_string () , rustflags) ; let mut cargo_command : Vec < & dyn AsRef < OsStr > > = vec ! [& "cargo" , & toolchain_arg] ; cargo_command . extend_from_slice (command) ; callback (& cargo_command , cwd , & env) }
};
}
