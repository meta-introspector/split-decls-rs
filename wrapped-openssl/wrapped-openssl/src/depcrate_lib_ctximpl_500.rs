// Generated macro for impl_500 (impl)
macro_rules! Depcrate_lib_ctximpl_500 {
() => {
// Module: crate::lib_ctx
// Provides: {"impl_500"}
// Dependencies: {}
impl LibCtx { # [corresponds (OSSL_LIB_CTX_new)] pub fn new () -> Result < Self , ErrorStack > { unsafe { let ptr = cvt_p (ffi :: OSSL_LIB_CTX_new ()) ? ; Ok (LibCtx :: from_ptr (ptr)) } } }
};
}
