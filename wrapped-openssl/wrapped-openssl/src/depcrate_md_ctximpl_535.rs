// Generated macro for impl_535 (impl)
macro_rules! Depcrate_md_ctximpl_535 {
() => {
// Module: crate::md_ctx
// Provides: {"impl_535"}
// Dependencies: {}
impl MdCtx { # [doc = " Creates a new context."] # [corresponds (EVP_MD_CTX_new)] # [inline] pub fn new () -> Result < Self , ErrorStack > { ffi :: init () ; unsafe { let ptr = cvt_p (EVP_MD_CTX_new ()) ? ; Ok (MdCtx :: from_ptr (ptr)) } } }
};
}
