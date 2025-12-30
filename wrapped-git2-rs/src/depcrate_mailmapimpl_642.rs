// Generated macro for impl_642 (impl)
macro_rules! Depcrate_mailmapimpl_642 {
() => {
// Module: crate::mailmap
// Provides: {"impl_642"}
// Dependencies: {}
impl Binding for Mailmap { type Raw = * mut raw :: git_mailmap ; unsafe fn from_raw (ptr : * mut raw :: git_mailmap) -> Mailmap { Mailmap { raw : ptr } } fn raw (& self) -> * mut raw :: git_mailmap { self . raw } }
};
}
