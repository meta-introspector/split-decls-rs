macro_rules! deps {
    () => {
        DummyGitExecutor!();
        GitExecutor!();
        RollupLock!();
        SubmoduleStat!();
        Result!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl GitExecutor for DummyGitExecutor { fn submodule_add (& self , _repo_url : & str , _submodule_path : & Path , _rollup_lock : Arc < Mutex < RollupLock > > , _root_dir : & Path ,) -> Result < () > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: submodule_add is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: submodule_add is not implemented." ,))) ; } fn checkout_branch (& self , _submodule_path : & Path , _branch : & str , _rollup_lock : Arc < Mutex < RollupLock > > , _root_dir : & Path ,) -> Result < () > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: checkout_branch is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: checkout_branch is not implemented." ,))) ; } fn status (& self , _submodule_path : & Path) -> Result < String > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: status is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: status is not implemented." ,))) ; } fn list_submodules (& self , _root_dir : & Path) -> Result < Vec < (String , PathBuf) > > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: list_submodules is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: list_submodules is not implemented." ,))) ; } fn clone (& self , _repo_url : & str , _target_path : & Path) -> Result < () > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: clone is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: clone is not implemented." ,))) ; } fn get_file_git_info (& self , _repo_path : & Path , _file_path : & Path ,) -> Result < (bool , Option < String >) > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: get_file_git_info is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: get_file_git_info is not implemented." ,))) ; } fn get_submodule_head_and_workdir_hash (& self , _path : & Path) -> Result < SubmoduleStat > { # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("DummyGitExecutor: get_submodule_head_and_workdir_hash is not implemented.") ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "DummyGitExecutor: get_submodule_head_and_workdir_hash is not implemented." ,))) ; } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_3!()