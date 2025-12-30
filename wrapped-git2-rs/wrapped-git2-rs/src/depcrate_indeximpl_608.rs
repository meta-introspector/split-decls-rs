// Generated macro for impl_608 (impl)
macro_rules! Depcrate_indeximpl_608 {
() => {
// Module: crate::index
// Provides: {"impl_608"}
// Dependencies: {}
impl Binding for Index { type Raw = * mut raw :: git_index ; unsafe fn from_raw (raw : * mut raw :: git_index) -> Index { Index { raw } } fn raw (& self) -> * mut raw :: git_index { self . raw } }
};
}
