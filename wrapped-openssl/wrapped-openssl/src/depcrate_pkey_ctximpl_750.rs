// Generated macro for impl_750 (impl)
macro_rules! Depcrate_pkey_ctximpl_750 {
() => {
// Module: crate::pkey_ctx
// Provides: {"impl_750"}
// Dependencies: {}
impl < T > PkeyCtx < T > { # [doc = " Creates a new pkey context using the provided key."] # [corresponds (EVP_PKEY_CTX_new)] # [inline] pub fn new (pkey : & PKeyRef < T >) -> Result < Self , ErrorStack > { unsafe { let ptr = cvt_p (ffi :: EVP_PKEY_CTX_new (pkey . as_ptr () , ptr :: null_mut ())) ? ; Ok (PkeyCtx :: from_ptr (ptr)) } } }
};
}
