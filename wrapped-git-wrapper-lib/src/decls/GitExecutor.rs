macro_rules! deps {
    () => {
        SubmoduleStat!();
        Result!();
        RollupLock!();
    };
}

macro_rules! GitExecutor {
    () => {
        deps!();
        pub trait GitExecutor : Send + Sync { fn submodule_add (& self , repo_url : & str , submodule_path : & Path , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : & Path ,) -> Result < () > ; fn checkout_branch (& self , submodule_path : & Path , branch : & str , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : & Path ,) -> Result < () > ; fn status (& self , submodule_path : & Path) -> Result < String > ; fn list_submodules (& self , root_dir : & Path) -> Result < Vec < (String , PathBuf) > > ; fn clone (& self , repo_url : & str , target_path : & Path) -> Result < () > ; fn get_file_git_info (& self , repo_path : & Path , file_path : & Path ,) -> Result < (bool , Option < String >) > ; fn get_submodule_head_and_workdir_hash (& self , path : & Path) -> Result < SubmoduleStat > ; fn as_any (& self) -> & dyn Any ; }
    };
}

GitExecutor!()