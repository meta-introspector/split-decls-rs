/* FP:main.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_USE_0001
/* FP:main.rs-0002 */ use std :: { env , process } ;
/* FP:main.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0002
/* FP:main.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0003
/* FP:main.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0004
/* FP:main.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0005
/* FP:main.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0006
/* FP:main.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0007
/* FP:main.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0008
/* FP:main.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0009
/* FP:main.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0010
/* FP:main.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0011
/* FP:main.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0012
/* FP:main.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0013
/* FP:main.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MOD_0014
/* FP:main.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_CONST_0015
/* FP:main.rs-0030 */ const BUILD_DIR : & str = "build" ;
/* FP:main.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_MACRO_0016
/* FP:main.rs-0032 */ macro_rules ! arg_error { ($ ($ err : tt) *) => { { eprintln ! ($ ($ err) *) ; eprintln ! () ; usage () ; std :: process :: exit (1) ; } } ; }
/* FP:main.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_FN_0017
/* FP:main.rs-0034 */ fn usage () { println ! ("\
/* FP:main.rs-0035 */ rustc_codegen_gcc build system
/* FP:main.rs-0036 */ 
/* FP:main.rs-0037 */ Usage: build_system [command] [options]
/* FP:main.rs-0038 */ 
/* FP:main.rs-0039 */ Options:
/* FP:main.rs-0040 */         --help    : Displays this help message.
/* FP:main.rs-0041 */ 
/* FP:main.rs-0042 */ Commands:
/* FP:main.rs-0043 */         cargo     : Executes a cargo command.
/* FP:main.rs-0044 */         rustc     : Compiles the program using the GCC compiler.
/* FP:main.rs-0045 */         clean     : Cleans the build directory, removing all compiled files and artifacts.
/* FP:main.rs-0046 */         prepare   : Prepares the environment for building, including fetching dependencies and setting up configurations.
/* FP:main.rs-0047 */         build     : Compiles the project.
/* FP:main.rs-0048 */         test      : Runs tests for the project.
/* FP:main.rs-0049 */         info      : Displays information about the build environment and project configuration.
/* FP:main.rs-0050 */         clone-gcc : Clones the GCC compiler from a specified source.
/* FP:main.rs-0051 */         fmt       : Runs rustfmt
/* FP:main.rs-0052 */         fuzz      : Fuzzes `cg_gcc` using rustlantis
/* FP:main.rs-0053 */         abi-test   : Runs the abi-cafe test suite on the codegen, checking for ABI compatibility with LLVM") ; }
/* FP:main.rs-0054 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_ENUM_0018
/* FP:main.rs-0055 */ pub enum Command { Cargo , Clean , CloneGcc , Prepare , Build , Rustc , Test , Info , Fmt , Fuzz , AbiTest , }
/* FP:main.rs-0056 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_main_FN_0019
/* FP:main.rs-0057 */ fn main () { if env :: var ("RUST_BACKTRACE") . is_err () { unsafe { env :: set_var ("RUST_BACKTRACE" , "1") ; } } let command = match env :: args () . nth (1) . as_deref () { Some ("cargo") => Command :: Cargo , Some ("rustc") => Command :: Rustc , Some ("clean") => Command :: Clean , Some ("prepare") => Command :: Prepare , Some ("build") => Command :: Build , Some ("test") => Command :: Test , Some ("info") => Command :: Info , Some ("clone-gcc") => Command :: CloneGcc , Some ("abi-test") => Command :: AbiTest , Some ("fmt") => Command :: Fmt , Some ("fuzz") => Command :: Fuzz , Some ("--help") => { usage () ; process :: exit (0) ; } Some (flag) if flag . starts_with ('-') => arg_error ! ("Expected command found flag {}" , flag) , Some (command) => arg_error ! ("Unknown command {}" , command) , None => { usage () ; process :: exit (0) ; } } ; if let Err (e) = match command { Command :: Cargo => rust_tools :: run_cargo () , Command :: Rustc => rust_tools :: run_rustc () , Command :: Clean => clean :: run () , Command :: Prepare => prepare :: run () , Command :: Build => build :: run () , Command :: Test => test :: run () , Command :: Info => info :: run () , Command :: CloneGcc => clone_gcc :: run () , Command :: Fmt => fmt :: run () , Command :: Fuzz => fuzz :: run () , Command :: AbiTest => abi_test :: run () , } { eprintln ! ("Command failed to run: {e}") ; process :: exit (1) ; } }