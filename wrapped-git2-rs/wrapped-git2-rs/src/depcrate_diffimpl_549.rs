// Generated macro for impl_549 (impl)
macro_rules! Depcrate_diffimpl_549 {
() => {
// Module: crate::diff
// Provides: {"impl_549"}
// Dependencies: {}
impl Binding for DiffStats { type Raw = * mut raw :: git_diff_stats ; unsafe fn from_raw (raw : * mut raw :: git_diff_stats) -> DiffStats { DiffStats { raw } } fn raw (& self) -> * mut raw :: git_diff_stats { self . raw } }
};
}
