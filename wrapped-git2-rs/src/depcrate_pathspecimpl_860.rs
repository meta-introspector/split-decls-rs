// Generated macro for impl_860 (impl)
macro_rules! Depcrate_pathspecimpl_860 {
() => {
// Module: crate::pathspec
// Provides: {"impl_860"}
// Dependencies: {}
impl Binding for Pathspec { type Raw = * mut raw :: git_pathspec ; unsafe fn from_raw (raw : * mut raw :: git_pathspec) -> Pathspec { Pathspec { raw } } fn raw (& self) -> * mut raw :: git_pathspec { self . raw } }
};
}
