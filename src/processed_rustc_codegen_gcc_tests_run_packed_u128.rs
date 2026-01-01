/* FP:packed_u128.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_packed_u128_USE_0001
/* FP:packed_u128.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use intrinsics :: black_box ;
/* FP:packed_u128.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_packed_u128_USE_0002
/* FP:packed_u128.rs-0004 */ use mini_core :: * ;
/* FP:packed_u128.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_packed_u128_STRUCT_0003
/* FP:packed_u128.rs-0006 */ # [repr (packed (1))] pub struct ScalarInt { data : u128 , size : u8 , }
/* FP:packed_u128.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_packed_u128_FN_0004
/* FP:packed_u128.rs-0008 */ # [inline (never)] # [unsafe (no_mangle)] fn read_data (a : & ScalarInt) { black_box (a . data) ; }
/* FP:packed_u128.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_packed_u128_FN_0005
/* FP:packed_u128.rs-0010 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { let data = [black_box (ScalarInt { data : 0 , size : 1 }) , black_box (ScalarInt { data : 0 , size : 1 })] ; read_data (& data [1]) ; 0 }