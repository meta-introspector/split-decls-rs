// Generated macro for BuiltinSubmoduleStatus (struct)
macro_rules! Depcrate_status_index_worktreeBuiltinSubmoduleStatus {
() => {
// Module: crate::status::index_worktree
// Provides: {"BuiltinSubmoduleStatus"}
// Dependencies: {}
# [doc = " An implementation of a trait to use with [`Repository::index_worktree_status()`] to compute the submodule status"] # [doc = " using [Submodule::status()](crate::Submodule::status())."] # [derive (Clone)] pub struct BuiltinSubmoduleStatus { mode : crate :: status :: Submodule , # [cfg (feature = "parallel")] repo : crate :: ThreadSafeRepository , # [cfg (not (feature = "parallel"))] git_dir : std :: path :: PathBuf , submodule_paths : Vec < BString > , }
};
}
