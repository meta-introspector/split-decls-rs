// Generated macro for asm_tests (function)
macro_rules! Depcrate_testasm_tests {
() => {
// Module: crate::test
// Provides: {"asm_tests"}
// Dependencies: {}
fn asm_tests (env : & Env , args : & TestArg) -> Result < () , String > { let mut env = env . clone () ; let rust_dir = setup_rustc (& mut env , args) ? ; println ! ("[TEST] rustc asm test suite") ; let codegen_backend_path = format ! ("{pwd}/target/{channel}/librustc_codegen_gcc.{dylib_ext}" , pwd = std :: env :: current_dir () . map_err (| error | format ! ("`current_dir` failed: {error:?}")) ? . display () , channel = args . config_info . channel . as_str () , dylib_ext = args . config_info . dylib_ext ,) ; let extra = if args . is_using_gcc_master_branch () { "" } else { " -Csymbol-mangling-version=v0" } ; let rustc_args = format ! ("-Zpanic-abort-tests -Zcodegen-backend={codegen_backend_path} --sysroot {} -Cpanic=abort{extra}" , args . config_info . sysroot_path) ; run_command_with_env (& [& "./x.py" , & "test" , & "--run" , & "always" , & "--stage" , & "0" , & "--set" , & "build.compiletest-allow-stage0=true" , & "tests/assembly-llvm/asm" , & "--compiletest-rustc-args" , & rustc_args ,] , Some (& rust_dir) , Some (& env) ,) ? ; Ok (()) }
};
}
