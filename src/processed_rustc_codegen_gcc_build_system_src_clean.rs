/* FP:clean.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_USE_0001
/* FP:clean.rs-0002 */ use std :: fs :: remove_dir_all ;
/* FP:clean.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_USE_0002
/* FP:clean.rs-0004 */ use std :: path :: Path ;
/* FP:clean.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_USE_0003
/* FP:clean.rs-0006 */ use crate :: utils :: { get_sysroot_dir , remove_file , run_command } ;
/* FP:clean.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_ENUM_0004
/* FP:clean.rs-0008 */ # [derive (Default)] enum CleanArg { # [doc = " `clean all`"] All , # [doc = " `clean ui-tests`"] UiTests , # [doc = " `clean --help`"] # [default] Help , }
/* FP:clean.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_IMPL_0005
/* FP:clean.rs-0010 */ impl CleanArg { fn new () -> Result < Self , String > { if let Some (arg) = std :: env :: args () . nth (2) { return match arg . as_str () { "all" => Ok (Self :: All) , "ui-tests" => Ok (Self :: UiTests) , "--help" => Ok (Self :: Help) , a => Err (format ! ("Unknown argument `{a}`")) , } ; } Ok (Self :: default ()) } }
/* FP:clean.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_FN_0006
/* FP:clean.rs-0012 */ fn usage () { println ! (r#"
/* FP:clean.rs-0013 */ `clean` command help:
/* FP:clean.rs-0014 */ 
/* FP:clean.rs-0015 */     all                      : Clean all data
/* FP:clean.rs-0016 */     ui-tests                 : Clean ui tests
/* FP:clean.rs-0017 */     --help                   : Show this help
/* FP:clean.rs-0018 */ "#) }
/* FP:clean.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_FN_0007
/* FP:clean.rs-0020 */ fn clean_all () -> Result < () , String > { let build_sysroot = get_sysroot_dir () ; let dirs_to_remove = ["target" . into () , build_sysroot . join ("sysroot") , build_sysroot . join ("sysroot_src") , build_sysroot . join ("target") ,] ; for dir in dirs_to_remove { let _ = remove_dir_all (dir) ; } let dirs_to_remove = ["regex" , "rand" , "simple-raytracer"] ; for dir in dirs_to_remove { let _ = remove_dir_all (Path :: new (crate :: BUILD_DIR) . join (dir)) ; } let files_to_remove = [build_sysroot . join ("Cargo.lock") , "perf.data" . into () , "perf.data.old" . into ()] ; for file in files_to_remove { let _ = remove_file (& file) ; } println ! ("Successfully ran `clean all`") ; Ok (()) }
/* FP:clean.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_FN_0008
/* FP:clean.rs-0022 */ fn clean_ui_tests () -> Result < () , String > { let path = Path :: new (crate :: BUILD_DIR) . join ("rust/build/x86_64-unknown-linux-gnu/test/ui/") ; run_command (& [& "find" , & path , & "-name" , & "stamp" , & "-delete"] , None) ? ; Ok (()) }
/* FP:clean.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_clean_FN_0009
/* FP:clean.rs-0024 */ pub fn run () -> Result < () , String > { match CleanArg :: new () ? { CleanArg :: All => clean_all () ? , CleanArg :: UiTests => clean_ui_tests () ? , CleanArg :: Help => usage () , } Ok (()) }