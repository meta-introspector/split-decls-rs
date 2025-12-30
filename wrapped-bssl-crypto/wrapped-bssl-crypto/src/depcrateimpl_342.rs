// Generated macro for impl_342 (impl)
macro_rules! Depcrateimpl_342 {
() => {
// Module: crate
// Provides: {"impl_342"}
// Dependencies: {}
impl Drop for Buffer { fn drop (& mut self) { unsafe { bssl_sys :: OPENSSL_free (self . ptr as * mut core :: ffi :: c_void) ; } } }
};
}
