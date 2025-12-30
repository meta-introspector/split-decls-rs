// Generated macro for impl_20 (impl)
macro_rules! Depcrate_abiimpl_20 {
() => {
// Module: crate::abi
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Debug for BlockFlags { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("BlockFlags") ; f . field ("value" , & format ! ("{:032b}" , self . 0)) ; macro_rules ! test_flags { { $ ($ (# [$ m : meta]) ? $ name : ident : $ flag : ident) ;* $ (;) ? } => ($ ($ (# [$ m]) ? f . field (stringify ! ($ name) , & self . has (Self ::$ flag)) ;) *) } test_flags ! { # [cfg (target_vendor = "apple")] deallocating : BLOCK_DEALLOCATING ; # [cfg (target_vendor = "apple")] inline_layout_string : BLOCK_INLINE_LAYOUT_STRING ; # [cfg (target_vendor = "apple")] small_descriptor : BLOCK_SMALL_DESCRIPTOR ; # [cfg (target_vendor = "apple")] is_noescape : BLOCK_IS_NOESCAPE ; # [cfg (target_vendor = "apple")] needs_free : BLOCK_NEEDS_FREE ; has_copy_dispose : BLOCK_HAS_COPY_DISPOSE ; has_ctor : BLOCK_HAS_CTOR ; # [cfg (target_vendor = "apple")] is_gc : BLOCK_IS_GC ; is_global : BLOCK_IS_GLOBAL ; use_stret : BLOCK_USE_STRET ; has_signature : BLOCK_HAS_SIGNATURE ; # [cfg (target_vendor = "apple")] has_extended_layout : BLOCK_HAS_EXTENDED_LAYOUT ; } f . field ("over_referenced" , & self . has (Self :: BLOCK_REFCOUNT_MASK)) ; f . field ("reference_count" , & ((* self & Self :: BLOCK_REFCOUNT_MASK) . 0 >> 1) ,) ; f . finish_non_exhaustive () } }
};
}
