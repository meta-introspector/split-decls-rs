// Generated macro for impl_750 (impl)
macro_rules! Depcrate_open_repositoryimpl_750 {
() => {
// Module: crate::open::repository
// Provides: {"impl_750"}
// Dependencies: {}
impl EnvironmentOverrides { fn from_env () -> Result < Self , gix_sec :: permission :: Error < std :: path :: PathBuf > > { let mut worktree_dir = None ; if let Some (path) = std :: env :: var_os (Core :: WORKTREE . the_environment_override ()) { worktree_dir = PathBuf :: from (path) . into () ; } let mut git_dir = None ; if let Some (path) = std :: env :: var_os ("GIT_DIR") { git_dir = PathBuf :: from (path) . into () ; } Ok (EnvironmentOverrides { worktree_dir , git_dir }) } }
};
}
