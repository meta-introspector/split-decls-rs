/* FP:structs.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_structs_USE_0001
/* FP:structs.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:structs.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_structs_STRUCT_0002
/* FP:structs.rs-0004 */ struct Test { field : isize , }
/* FP:structs.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_structs_STRUCT_0003
/* FP:structs.rs-0006 */ struct Two { two : isize , }
/* FP:structs.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_structs_FN_0004
/* FP:structs.rs-0008 */ fn one () -> isize { 1 }
/* FP:structs.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_structs_FN_0005
/* FP:structs.rs-0010 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { let test = Test { field : one () } ; let two = Two { two : 2 } ; unsafe { libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , test . field) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , two . two) ; } 0 }