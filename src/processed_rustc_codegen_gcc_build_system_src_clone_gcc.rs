/* FP:clone_gcc.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_USE_0001
/* FP:clone_gcc.rs-0002 */ use std :: path :: { Path , PathBuf } ;
/* FP:clone_gcc.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_USE_0002
/* FP:clone_gcc.rs-0004 */ use crate :: config :: ConfigInfo ;
/* FP:clone_gcc.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_USE_0003
/* FP:clone_gcc.rs-0006 */ use crate :: utils :: { git_clone , run_command_with_output } ;
/* FP:clone_gcc.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_FN_0004
/* FP:clone_gcc.rs-0008 */ fn show_usage () { println ! (r#"
/* FP:clone_gcc.rs-0009 */ `clone-gcc` command help:
/* FP:clone_gcc.rs-0010 */ 
/* FP:clone_gcc.rs-0011 */     --out-path         : Location where the GCC repository will be cloned (default: `./gcc`)"#) ; ConfigInfo :: show_usage () ; println ! ("    --help                 : Show this help") ; }
/* FP:clone_gcc.rs-0012 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_STRUCT_0005
/* FP:clone_gcc.rs-0013 */ # [derive (Default)] struct Args { out_path : PathBuf , config_info : ConfigInfo , }
/* FP:clone_gcc.rs-0014 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_IMPL_0006
/* FP:clone_gcc.rs-0015 */ impl Args { fn new () -> Result < Option < Self > , String > { let mut command_args = Self :: default () ; let mut out_path = None ; let mut args = std :: env :: args () . skip (2) ; while let Some (arg) = args . next () { match arg . as_str () { "--out-path" => match args . next () { Some (path) if ! path . is_empty () => out_path = Some (path) , _ => { return Err ("Expected an argument after `--out-path`, found nothing" . into ()) ; } } , "--help" => { show_usage () ; return Ok (None) ; } arg => { if ! command_args . config_info . parse_argument (arg , & mut args) ? { return Err (format ! ("Unknown option {arg}")) ; } } } } command_args . out_path = match out_path { Some (p) => p . into () , None => PathBuf :: from ("./gcc") , } ; Ok (Some (command_args)) } }
/* FP:clone_gcc.rs-0016 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clone_gcc_FN_0007
/* FP:clone_gcc.rs-0017 */ pub fn run () -> Result < () , String > { let Some (args) = Args :: new () ? else { return Ok (()) ; } ; let result = git_clone ("https://github.com/rust-lang/gcc" , Some (& args . out_path) , false) ? ; if result . ran_clone { let gcc_commit = args . config_info . get_gcc_commit () ? ; println ! ("Checking out GCC commit `{gcc_commit}`...") ; run_command_with_output (& [& "git" , & "checkout" , & gcc_commit] , Some (Path :: new (& result . repo_dir)) ,) ? ; } else { println ! ("There is already a GCC folder in `{}`, leaving things as is..." , args . out_path . display ()) ; } Ok (()) }