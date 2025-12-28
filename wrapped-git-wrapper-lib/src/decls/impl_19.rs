macro_rules! deps {
    () => {
        SubmoduleStat!();
        MockGitAdapter!();
        Result!();
        GitAdapter!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl GitAdapter for MockGitAdapter { fn list_submodules (& self , _root_dir : & Path) -> Result < Vec < (String , PathBuf) > > { println ! ("[MockGitAdapter] Listing submodules (mock data)") ; Ok (self . mock_submodules . clone ()) } fn get_submodule_head_and_workdir_hash (& self , path : & Path) -> Result < SubmoduleStat > { println ! ("[MockGitAdapter] Getting submodule stat for {:?}" , path) ; Ok (self . mock_submodule_stat . clone ()) } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_19!();