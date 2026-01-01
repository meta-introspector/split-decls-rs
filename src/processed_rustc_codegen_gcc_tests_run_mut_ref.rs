/* FP:mut_ref.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_mut_ref_USE_0001
/* FP:mut_ref.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:mut_ref.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_mut_ref_STRUCT_0002
/* FP:mut_ref.rs-0004 */ struct Test { field : isize , }
/* FP:mut_ref.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_mut_ref_FN_0003
/* FP:mut_ref.rs-0006 */ fn test (num : isize) -> Test { Test { field : num + 1 } }
/* FP:mut_ref.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_mut_ref_FN_0004
/* FP:mut_ref.rs-0008 */ fn update_num (num : & mut isize) { * num = * num + 5 ; }
/* FP:mut_ref.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_mut_ref_FN_0005
/* FP:mut_ref.rs-0010 */ # [unsafe (no_mangle)] extern "C" fn main (mut argc : isize , _argv : * const * const u8) -> i32 { let mut test = test (argc) ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , test . field) ; } update_num (& mut test . field) ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , test . field) ; } update_num (& mut argc) ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , argc) ; } let refe = & mut argc ; * refe = * refe + 5 ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , argc) ; } 0 }