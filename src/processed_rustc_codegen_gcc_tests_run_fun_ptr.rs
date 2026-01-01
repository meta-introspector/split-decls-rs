/* FP:fun_ptr.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_fun_ptr_USE_0001
/* FP:fun_ptr.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:fun_ptr.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_fun_ptr_FN_0002
/* FP:fun_ptr.rs-0004 */ fn i16_as_i8 (a : i16) -> i8 { a as i8 }
/* FP:fun_ptr.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_fun_ptr_FN_0003
/* FP:fun_ptr.rs-0006 */ fn call_func (func : fn (i16) -> i8 , param : i16) -> i8 { func (param) }
/* FP:fun_ptr.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_fun_ptr_FN_0004
/* FP:fun_ptr.rs-0008 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { unsafe { let result = call_func (i16_as_i8 , argc as i16) as isize ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , result) ; } 0 }