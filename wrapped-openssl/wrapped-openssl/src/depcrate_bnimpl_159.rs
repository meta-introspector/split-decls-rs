// Generated macro for impl_159 (impl)
macro_rules! Depcrate_bnimpl_159 {
() => {
// Module: crate::bn
// Provides: {"impl_159"}
// Dependencies: {}
impl BigNumContext { # [doc = " Returns a new `BigNumContext`."] # [corresponds (BN_CTX_new)] pub fn new () -> Result < BigNumContext , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: BN_CTX_new ()) . map (BigNumContext) } } # [doc = " Returns a new secure `BigNumContext`."] # [corresponds (BN_CTX_secure_new)] # [cfg (ossl110)] pub fn new_secure () -> Result < BigNumContext , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: BN_CTX_secure_new ()) . map (BigNumContext) } } }
};
}
