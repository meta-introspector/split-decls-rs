macro_rules! deps {
    () => {
        SystemGitExecutor!();
        SubmoduleStat!();
        Result!();
        GitAdapter!();
        GitExecutor!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl GitAdapter for SystemGitExecutor { fn list_submodules (& self , root_dir : & Path) -> Result < Vec < (String , PathBuf) > > { GitExecutor :: list_submodules (self , root_dir) } fn get_submodule_head_and_workdir_hash (& self , path : & Path) -> Result < SubmoduleStat > { GitExecutor :: get_submodule_head_and_workdir_hash (self , path) } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_91!();