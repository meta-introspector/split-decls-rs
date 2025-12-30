// Generated macro for impl_672 (impl)
macro_rules! Depcrate_mergeimpl_672 {
() => {
// Module: crate::merge
// Provides: {"impl_672"}
// Dependencies: {}
impl < 'repo > Drop for AnnotatedCommit < 'repo > { fn drop (& mut self) { unsafe { raw :: git_annotated_commit_free (self . raw) } } }
};
}
