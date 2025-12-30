// Generated macro for impl_751 (impl)
macro_rules! Depcrate_pkey_ctximpl_751 {
() => {
// Module: crate::pkey_ctx
// Provides: {"impl_751"}
// Dependencies: {}
impl PkeyCtx < () > { # [doc = " Creates a new pkey context for the specified algorithm ID."] # [corresponds (EVP_PKEY_CTX_new_id)] # [inline] pub fn new_id (id : Id) -> Result < Self , ErrorStack > { unsafe { let ptr = cvt_p (ffi :: EVP_PKEY_CTX_new_id (id . as_raw () , ptr :: null_mut ())) ? ; Ok (PkeyCtx :: from_ptr (ptr)) } } }
};
}
