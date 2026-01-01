/* FP:static.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_USE_0001
/* FP:static.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:static.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_STRUCT_0002
/* FP:static.rs-0004 */ struct Test { field : isize , }
/* FP:static.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_STRUCT_0003
/* FP:static.rs-0006 */ struct WithRef { refe : & 'static Test , }
/* FP:static.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_STATIC_0004
/* FP:static.rs-0008 */ static mut CONSTANT : isize = 10 ;
/* FP:static.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_STATIC_0005
/* FP:static.rs-0010 */ static mut TEST : Test = Test { field : 12 } ;
/* FP:static.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_STATIC_0006
/* FP:static.rs-0012 */ static mut TEST2 : Test = Test { field : 14 } ;
/* FP:static.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_STATIC_0007
/* FP:static.rs-0014 */ static mut WITH_REF : WithRef = WithRef { refe : unsafe { & TEST } } ;
/* FP:static.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_static_FN_0008
/* FP:static.rs-0016 */ # [unsafe (no_mangle)] extern "C" fn main (argc : isize , _argv : * const * const u8) -> i32 { unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , CONSTANT) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , TEST2 . field) ; TEST2 . field = argc ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , TEST2 . field) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , WITH_REF . refe . field) ; WITH_REF . refe = & TEST2 ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , TEST . field) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , WITH_REF . refe . field) ; } 0 }