// Generated macro for impl_498 (impl)
macro_rules! Depcrate_repository_implsimpl_498 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_498"}
// Dependencies: {}
impl From < & crate :: ThreadSafeRepository > for crate :: Repository { fn from (repo : & crate :: ThreadSafeRepository) -> Self { crate :: Repository :: from_refs_and_objects (repo . refs . clone () , gix_odb :: memory :: Proxy :: from (gix_odb :: Cache :: from (repo . objects . to_handle ())) . with_write_passthrough () , repo . work_tree . clone () , repo . common_dir . clone () , repo . config . clone () , repo . linked_worktree_options . clone () , # [cfg (feature = "index")] repo . index . clone () , repo . shallow_commits . clone () , # [cfg (feature = "attributes")] repo . modules . clone () ,) } }
};
}
