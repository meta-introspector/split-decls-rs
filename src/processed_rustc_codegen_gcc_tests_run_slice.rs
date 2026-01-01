/* FP:slice.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_slice_USE_0001
/* FP:slice.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:slice.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_slice_STATIC_0002
/* FP:slice.rs-0004 */ static mut TWO : usize = 2 ;
/* FP:slice.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_slice_FN_0003
/* FP:slice.rs-0006 */ fn index_slice (s : & [u32]) -> u32 { unsafe { s [TWO] } }
/* FP:slice.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_slice_FN_0004
/* FP:slice.rs-0008 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { let array = [42 , 7 , 5] ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , index_slice (& array)) ; } 0 }