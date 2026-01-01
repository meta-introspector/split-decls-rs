/* FP:tlv.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_USE_0001
/* FP:tlv.rs-0002 */ use std :: cell :: Cell ;
/* FP:tlv.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_USE_0002
/* FP:tlv.rs-0004 */ use std :: ptr ;
/* FP:tlv.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_MACRO_0003
/* FP:tlv.rs-0006 */ thread_local ! (pub static TLV : Cell <* const () > = const { Cell :: new (ptr :: null ()) }) ;
/* FP:tlv.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_STRUCT_0004
/* FP:tlv.rs-0008 */ # [derive (Copy , Clone)] pub (crate) struct Tlv (pub (crate) * const ()) ;
/* FP:tlv.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_IMPL_0005
/* FP:tlv.rs-0010 */ impl Tlv { # [inline] pub (crate) fn null () -> Self { Self (ptr :: null ()) } }
/* FP:tlv.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_IMPL_0006
/* FP:tlv.rs-0012 */ unsafe impl Sync for Tlv { }
/* FP:tlv.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_IMPL_0007
/* FP:tlv.rs-0014 */ unsafe impl Send for Tlv { }
/* FP:tlv.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_FN_0008
/* FP:tlv.rs-0016 */ # [doc = " Sets the current thread-local value"] # [inline] pub (crate) fn set (value : Tlv) { TLV . with (| tlv | tlv . set (value . 0)) ; }
/* FP:tlv.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_tlv_FN_0009
/* FP:tlv.rs-0018 */ # [doc = " Returns the current thread-local value"] # [inline] pub (crate) fn get () -> Tlv { TLV . with (| tlv | Tlv (tlv . get ())) }