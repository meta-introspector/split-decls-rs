macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! RepoStateCollector {
    () => {
        deps!();
        pub trait RepoStateCollector { fn collect_repo_state (& self , project_root : & Path) -> Result < RepoState > ; }
    };
}

RepoStateCollector!()