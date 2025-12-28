macro_rules! deps {
    () => {
        SubmoduleStat!();
        Result!();
    };
}

macro_rules! GitAdapter {
    () => {
        deps!();
        # [doc = " A unified trait for Git operations, abstracting different execution modes."] pub trait GitAdapter : Send + Sync { # [doc = " Lists submodules in the given root directory."] fn list_submodules (& self , root_dir : & Path) -> Result < Vec < (String , PathBuf) > > ; # [doc = " Gets the head and workdir hash for a submodule."] fn get_submodule_head_and_workdir_hash (& self , path : & Path) -> Result < SubmoduleStat > ; # [doc = " Returns a reference to `Any` for downcasting."] fn as_any (& self) -> & dyn Any ; }
    };
}

GitAdapter!();