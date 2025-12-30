// Generated macro for impl_671 (impl)
macro_rules! Depcrate_mergeimpl_671 {
() => {
// Module: crate::merge
// Provides: {"impl_671"}
// Dependencies: {}
impl < 'repo > Binding for AnnotatedCommit < 'repo > { type Raw = * mut raw :: git_annotated_commit ; unsafe fn from_raw (raw : * mut raw :: git_annotated_commit) -> AnnotatedCommit < 'repo > { AnnotatedCommit { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_annotated_commit { self . raw } }
};
}
