/* FP:array.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_array_USE_0001
/* FP:array.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:array.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_array_STATIC_0002
/* FP:array.rs-0004 */ static mut ONE : usize = 1 ;
/* FP:array.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_array_FN_0003
/* FP:array.rs-0006 */ fn make_array () -> [u8 ; 3] { [42 , 10 , 5] }
/* FP:array.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_array_FN_0004
/* FP:array.rs-0008 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { let array = [42 , 7 , 5] ; let array2 = make_array () ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , array [ONE - 1]) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , array [ONE]) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , array [ONE + 1]) ; libc :: printf (b"%d\n\0" as * const u8 as * const i8 , array2 [argc as usize] as u32) ; } 0 }