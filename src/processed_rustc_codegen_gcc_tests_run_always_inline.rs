/* FP:always_inline.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_always_inline_USE_0001
/* FP:always_inline.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:always_inline.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_always_inline_FN_0002
/* FP:always_inline.rs-0004 */ # [inline (always)] fn fib (n : u8) -> u8 { if n == 0 { return 1 ; } if n == 1 { return 1 ; } fib (n - 1) + fib (n - 2) }
/* FP:always_inline.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_always_inline_FN_0003
/* FP:always_inline.rs-0006 */ # [inline (always)] fn fib_b (n : u8) -> u8 { if n == 0 { return 1 ; } if n == 1 { return 1 ; } fib_a (n - 1) + fib_a (n - 2) }
/* FP:always_inline.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_always_inline_FN_0004
/* FP:always_inline.rs-0008 */ # [inline (always)] fn fib_a (n : u8) -> u8 { if n == 0 { return 1 ; } if n == 1 { return 1 ; } fib_b (n - 1) + fib_b (n - 2) }
/* FP:always_inline.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_always_inline_FN_0005
/* FP:always_inline.rs-0010 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { if fib (2) != fib_a (2) { intrinsics :: abort () ; } 0 }