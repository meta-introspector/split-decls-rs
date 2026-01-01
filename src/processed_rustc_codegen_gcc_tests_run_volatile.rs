/* FP:volatile.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_volatile_USE_0001
/* FP:volatile.rs-0002 */ use std :: mem :: MaybeUninit ;
/* FP:volatile.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_volatile_STRUCT_0002
/* FP:volatile.rs-0004 */ # [allow (dead_code)] # [derive (Debug)] struct Struct { pointer : * const () , func : unsafe fn (* const ()) , }
/* FP:volatile.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_volatile_FN_0003
/* FP:volatile.rs-0006 */ fn func (_ptr : * const ()) { }
/* FP:volatile.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_tests_run_volatile_FN_0004
/* FP:volatile.rs-0008 */ fn main () { let mut x = MaybeUninit :: < & Struct > :: uninit () ; x . write (& Struct { pointer : std :: ptr :: null () , func }) ; let x = unsafe { x . assume_init () } ; let value = unsafe { (x as * const Struct) . read_volatile () } ; println ! ("{:?}" , value) ; }