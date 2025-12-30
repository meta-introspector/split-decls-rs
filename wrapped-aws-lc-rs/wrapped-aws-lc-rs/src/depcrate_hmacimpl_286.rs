// Generated macro for impl_286 (impl)
macro_rules! Depcrate_hmacimpl_286 {
() => {
// Module: crate::hmac
// Provides: {"impl_286"}
// Dependencies: {}
impl LcHmacCtx { fn as_mut_ptr (& mut self) -> * mut HMAC_CTX { & mut self . 0 } fn as_ptr (& self) -> * const HMAC_CTX { & self . 0 } fn try_clone (& self) -> Result < Self , Unspecified > { unsafe { let mut hmac_ctx = MaybeUninit :: < HMAC_CTX > :: uninit () ; HMAC_CTX_init (hmac_ctx . as_mut_ptr ()) ; let mut hmac_ctx = hmac_ctx . assume_init () ; if 1 != HMAC_CTX_copy_ex (& mut hmac_ctx , self . as_ptr ()) { return Err (Unspecified) ; } Ok (LcHmacCtx (hmac_ctx)) } } }
};
}
