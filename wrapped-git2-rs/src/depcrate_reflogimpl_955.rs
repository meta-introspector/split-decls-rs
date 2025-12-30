// Generated macro for impl_955 (impl)
macro_rules! Depcrate_reflogimpl_955 {
() => {
// Module: crate::reflog
// Provides: {"impl_955"}
// Dependencies: {}
impl Binding for Reflog { type Raw = * mut raw :: git_reflog ; unsafe fn from_raw (raw : * mut raw :: git_reflog) -> Reflog { Reflog { raw } } fn raw (& self) -> * mut raw :: git_reflog { self . raw } }
};
}
