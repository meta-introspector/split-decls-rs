// Generated macro for impl_677 (impl)
macro_rules! Depcrate_mergeimpl_677 {
() => {
// Module: crate::merge
// Provides: {"impl_677"}
// Dependencies: {}
impl Drop for MergeFileResult { fn drop (& mut self) { unsafe { raw :: git_merge_file_result_free (& mut self . raw) } } }
};
}
