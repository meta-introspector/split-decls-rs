macro_rules! deps {
    () => {
        Core!();
        Error!();
        EnvironmentOverrides!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl EnvironmentOverrides { fn from_env () -> Result < Self , gix_sec :: permission :: Error < std :: path :: PathBuf > > { let mut worktree_dir = None ; if let Some (path) = std :: env :: var_os (Core :: WORKTREE . the_environment_override ()) { worktree_dir = PathBuf :: from (path) . into () ; } let mut git_dir = None ; if let Some (path) = std :: env :: var_os ("GIT_DIR") { git_dir = PathBuf :: from (path) . into () ; } Ok (EnvironmentOverrides { worktree_dir , git_dir }) } }
    };
}

impl_517!()