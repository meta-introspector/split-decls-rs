/* FP:ptr_cast.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_ptr_cast_USE_0001
/* FP:ptr_cast.rs-0002 */ # [feature (no_core)] # [no_std] # [no_core] # [no_main] use mini_core :: * ;
/* FP:ptr_cast.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_ptr_cast_FN_0002
/* FP:ptr_cast.rs-0004 */ fn int_cast (a : u16 , b : i16) -> (u8 , u16 , u32 , usize , i8 , i16 , i32 , isize , u8 , u32) { (a as u8 , a as u16 , a as u32 , a as usize , a as i8 , a as i16 , a as i32 , a as isize , b as u8 , b as u32 ,) }
/* FP:ptr_cast.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_ptr_cast_STATIC_0003
/* FP:ptr_cast.rs-0006 */ static mut ONE : usize = 1 ;
/* FP:ptr_cast.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_ptr_cast_FN_0004
/* FP:ptr_cast.rs-0008 */ # [unsafe (no_mangle)] extern "C" fn main (argc : i32 , _argv : * const * const u8) -> i32 { let (a , b , c , d , e , f , g , h , i , j) = int_cast (10 , 42) ; unsafe { libc :: printf (b"%d\n\0" as * const u8 as * const i8 , c) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , d) ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , j) ; let ptr = ONE as * mut usize ; let value = ptr as usize ; libc :: printf (b"%ld\n\0" as * const u8 as * const i8 , value) ; } 0 }