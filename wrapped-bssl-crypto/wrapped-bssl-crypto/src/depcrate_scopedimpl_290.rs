// Generated macro for impl_290 (impl)
macro_rules! Depcrate_scopedimpl_290 {
() => {
// Module: crate::scoped
// Provides: {"impl_290"}
// Dependencies: {}
impl EcKey { pub fn new () -> Self { let ptr = unsafe { bssl_sys :: EC_KEY_new () } ; assert ! (! ptr . is_null ()) ; EcKey (ptr) } pub fn as_ffi_ptr (& mut self) -> * mut bssl_sys :: EC_KEY { self . 0 } }
};
}
