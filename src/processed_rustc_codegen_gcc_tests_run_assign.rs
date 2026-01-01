/* FP:assign.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_assign_USE_0001
/* FP:assign.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:assign.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_assign_FN_0002
/* FP:assign.rs-0004 */ fn inc_ref (num : & mut isize) -> isize { * num = * num + 5 ; * num + 1 }
/* FP:assign.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_assign_FN_0003
/* FP:assign.rs-0006 */ fn inc (num : isize) -> isize { num + 1 }
/* FP:assign.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_assign_FN_0004
/* FP:assign.rs-0008 */ # [unsafe (no_mangle)] extern "C" fn main (mut argc : isize , _argv : * const * const u8) -> i32 { argc = inc (argc) ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , argc) ; } let b = inc_ref (& mut argc) ; unsafe { libc :: printf (b"%ld %ld\n\0" as * const u8 as * const i8 , argc , b) ; } argc = 10 ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , argc) ; } 0 }