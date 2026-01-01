/* FP:tuple.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_tuple_USE_0001
/* FP:tuple.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:tuple.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_tuple_FN_0002
/* FP:tuple.rs-0004 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { let test : (isize , isize , isize) = (3 , 1 , 4) ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , test . 0) ; } 0 }