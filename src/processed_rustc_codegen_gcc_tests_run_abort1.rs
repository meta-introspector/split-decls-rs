/* FP:abort1.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_abort1_USE_0001
/* FP:abort1.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:abort1.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_abort1_FN_0002
/* FP:abort1.rs-0004 */ fn test_fail () -> ! { unsafe { intrinsics :: abort () } ; }
/* FP:abort1.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_abort1_FN_0003
/* FP:abort1.rs-0006 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { test_fail () ; }