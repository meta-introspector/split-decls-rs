// Generated macro for impl_236 (impl)
macro_rules! Depcrate_cipher_ctximpl_236 {
() => {
// Module: crate::cipher_ctx
// Provides: {"impl_236"}
// Dependencies: {}
impl CipherCtx { # [doc = " Creates a new context."] # [corresponds (EVP_CIPHER_CTX_new)] pub fn new () -> Result < Self , ErrorStack > { ffi :: init () ; unsafe { let ptr = cvt_p (ffi :: EVP_CIPHER_CTX_new ()) ? ; Ok (CipherCtx :: from_ptr (ptr)) } } }
};
}
