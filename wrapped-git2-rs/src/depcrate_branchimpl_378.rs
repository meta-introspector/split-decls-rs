// Generated macro for impl_378 (impl)
macro_rules! Depcrate_branchimpl_378 {
() => {
// Module: crate::branch
// Provides: {"impl_378"}
// Dependencies: {}
impl < 'repo > Drop for Branches < 'repo > { fn drop (& mut self) { unsafe { raw :: git_branch_iterator_free (self . raw) } } }
};
}
