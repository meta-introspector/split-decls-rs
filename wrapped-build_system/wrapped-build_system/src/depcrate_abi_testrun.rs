// Generated macro for run (function)
macro_rules! Depcrate_abi_testrun {
() => {
// Module: crate::abi_test
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Result < () , String > { let mut args = std :: env :: args () . skip (2) ; # [allow (clippy :: never_loop , clippy :: while_let_on_iterator)] while let Some (arg) = args . next () { match arg . as_str () { "--help" => { show_usage () ; return Ok (()) ; } _ => return Err (format ! ("Unknown option {arg:?}")) , } } crate :: utils :: git_clone ("https://github.com/Gankra/abi-cafe.git" , Some ("clones/abi-cafe" . as_ref ()) , true ,) . map_err (| err | format ! ("Git clone failed with message: {err:?}!")) ? ; std :: fs :: copy ("rust-toolchain" , "clones/abi-cafe/rust-toolchain") . expect ("Could not copy toolchain configs!") ; let backend_path = std :: path :: absolute ("target/debug/librustc_codegen_gcc.so") . unwrap () ; let backend_arg = format ! ("--add-rustc-codegen-backend=cg_gcc:{}" , backend_path . display ()) ; let cmd : & [& dyn AsRef < OsStr >] = & [& "cargo" , & "run" , & "--release" , & "--" , & backend_arg , & "--pairs" , & "rustc_calls_cg_gcc" , & "--pairs" , & "cg_gcc_calls_rustc" , & "--pairs" , & "cg_gcc_calls_c" , & "--pairs" , & "c_calls_cg_gcc" ,] ; run_command_with_output (cmd , Some (Path :: new ("clones/abi-cafe"))) ? ; Ok (()) }
};
}
