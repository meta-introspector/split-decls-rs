// Generated macro for impl_1155 (impl)
macro_rules! Depcrate_worktree_proxyimpl_1155 {
() => {
// Module: crate::worktree::proxy
// Provides: {"impl_1155"}
// Dependencies: {}
impl < 'repo > Proxy < 'repo > { pub (crate) fn new (parent : & 'repo Repository , git_dir : impl Into < PathBuf >) -> Self { Proxy { parent , git_dir : git_dir . into () , } } pub (crate) fn new_if_gitdir_file_exists (parent : & 'repo Repository , git_dir : impl Into < PathBuf >) -> Option < Self > { let git_dir = git_dir . into () ; if git_dir . join ("gitdir") . is_file () { Some (Proxy :: new (parent , git_dir)) } else { None } } }
};
}
