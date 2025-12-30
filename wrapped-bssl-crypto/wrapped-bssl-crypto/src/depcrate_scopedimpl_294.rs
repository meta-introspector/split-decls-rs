// Generated macro for impl_294 (impl)
macro_rules! Depcrate_scopedimpl_294 {
() => {
// Module: crate::scoped
// Provides: {"impl_294"}
// Dependencies: {}
impl EvpHpkeCtx { pub fn new () -> Self { let ptr = unsafe { bssl_sys :: EVP_HPKE_CTX_new () } ; assert ! (! ptr . is_null ()) ; EvpHpkeCtx (ptr) } pub fn as_ffi_ptr (& self) -> * const bssl_sys :: EVP_HPKE_CTX { self . 0 } pub fn as_mut_ffi_ptr (& mut self) -> * mut bssl_sys :: EVP_HPKE_CTX { self . 0 } }
};
}
