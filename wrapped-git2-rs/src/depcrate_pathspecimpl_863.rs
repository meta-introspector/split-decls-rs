// Generated macro for impl_863 (impl)
macro_rules! Depcrate_pathspecimpl_863 {
() => {
// Module: crate::pathspec
// Provides: {"impl_863"}
// Dependencies: {}
impl < 'ps > Binding for PathspecMatchList < 'ps > { type Raw = * mut raw :: git_pathspec_match_list ; unsafe fn from_raw (raw : * mut raw :: git_pathspec_match_list) -> PathspecMatchList < 'ps > { PathspecMatchList { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_pathspec_match_list { self . raw } }
};
}
