macro_rules! deps {
    () => {
        RollupLock!();
        SubmoduleManager!();
        GitWrapperLibTrait!();
        Result!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl SubmoduleManager { pub fn new (git_wrapper : Arc < dyn GitWrapperLibTrait + Send + Sync > , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf ,) -> Self { SubmoduleManager { git_wrapper , rollup_lock , root_dir , } } pub fn add_submodule (& self , repo_url : & str , path : & Path) -> Result < () > { self . git_wrapper . git_repo_operations () . git_submodule_add (& self . root_dir , repo_url , path . to_str () . unwrap () , None , None ,) ? ; Ok (()) } pub fn update_submodules (& self) -> Result < () > { self . git_wrapper . git_repo_operations () . git_submodule_update (& self . root_dir , true , true) ? ; Ok (()) } pub fn status_submodules (& self) -> Result < String > { let output = self . git_wrapper . git_repo_operations () . git_submodule_status (& self . root_dir) ? ; Ok (String :: from_utf8_lossy (& output . stdout) . to_string ()) } }
    };
}

impl_75!();