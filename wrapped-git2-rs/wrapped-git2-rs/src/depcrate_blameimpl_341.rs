// Generated macro for impl_341 (impl)
macro_rules! Depcrate_blameimpl_341 {
() => {
// Module: crate::blame
// Provides: {"impl_341"}
// Dependencies: {}
impl Binding for BlameOptions { type Raw = * mut raw :: git_blame_options ; unsafe fn from_raw (opts : * mut raw :: git_blame_options) -> BlameOptions { BlameOptions { raw : * opts } } fn raw (& self) -> * mut raw :: git_blame_options { & self . raw as * const _ as * mut _ } }
};
}
