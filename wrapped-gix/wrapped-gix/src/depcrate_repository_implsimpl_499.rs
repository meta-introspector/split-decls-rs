// Generated macro for impl_499 (impl)
macro_rules! Depcrate_repository_implsimpl_499 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_499"}
// Dependencies: {}
impl From < crate :: ThreadSafeRepository > for crate :: Repository { fn from (repo : crate :: ThreadSafeRepository) -> Self { crate :: Repository :: from_refs_and_objects (repo . refs , gix_odb :: memory :: Proxy :: from (gix_odb :: Cache :: from (repo . objects . to_handle ())) . with_write_passthrough () , repo . work_tree , repo . common_dir , repo . config , repo . linked_worktree_options , # [cfg (feature = "index")] repo . index , repo . shallow_commits , # [cfg (feature = "attributes")] repo . modules . clone () ,) } }
};
}
