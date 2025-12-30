// Generated macro for impl_297 (impl)
macro_rules! Depcrate_scopedimpl_297 {
() => {
// Module: crate::scoped
// Provides: {"impl_297"}
// Dependencies: {}
impl EvpHpkeKey { pub fn new () -> Self { EvpHpkeKey (unsafe { initialized_struct (| ptr | bssl_sys :: EVP_HPKE_KEY_zero (ptr)) }) } pub fn as_ffi_ptr (& self) -> * const bssl_sys :: EVP_HPKE_KEY { & self . 0 } pub fn as_mut_ffi_ptr (& mut self) -> * mut bssl_sys :: EVP_HPKE_KEY { & mut self . 0 } }
};
}
