macro_rules! deps {
    () => {
        RepoSyncConfig!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl RepoSyncConfig { pub fn load_from_file (_path : & Path) -> Result < Self > { unimplemented ! () } }
    };
}

impl_147!()