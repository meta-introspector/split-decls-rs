// Generated macro for impl_461 (impl)
macro_rules! Depcrate_credimpl_461 {
() => {
// Module: crate::cred
// Provides: {"impl_461"}
// Dependencies: {}
impl Binding for Cred { type Raw = * mut raw :: git_cred ; unsafe fn from_raw (raw : * mut raw :: git_cred) -> Cred { Cred { raw } } fn raw (& self) -> * mut raw :: git_cred { self . raw } }
};
}
