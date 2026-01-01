/* FP:fatal_error.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_fatal_error_STRUCT_0001
/* FP:fatal_error.rs-0002 */ # [doc = " Used as a return value to signify a fatal error occurred."] # [derive (Copy , Clone , Debug)] # [must_use] pub struct FatalError ;
/* FP:fatal_error.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_fatal_error_USE_0002
/* FP:fatal_error.rs-0004 */ pub use crate :: rustc_data_structures :: FatalErrorMarker ;
/* FP:fatal_error.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_fatal_error_IMPL_0003
/* FP:fatal_error.rs-0006 */ impl ! Send for FatalError { }
/* FP:fatal_error.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_fatal_error_IMPL_0004
/* FP:fatal_error.rs-0008 */ impl FatalError { pub fn raise (self) -> ! { std :: panic :: resume_unwind (Box :: new (FatalErrorMarker)) } }
/* FP:fatal_error.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_fatal_error_IMPL_0005
/* FP:fatal_error.rs-0010 */ impl std :: fmt :: Display for FatalError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "fatal error") } }
/* FP:fatal_error.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_fatal_error_IMPL_0006
/* FP:fatal_error.rs-0012 */ impl std :: error :: Error for FatalError { }