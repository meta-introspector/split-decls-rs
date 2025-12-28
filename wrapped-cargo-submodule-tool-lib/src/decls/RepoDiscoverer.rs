macro_rules! RepoDiscoverer {
    () => {
        pub trait RepoDiscoverer { fn discover_repos (& self , root_dir : & Path ,) -> std :: result :: Result < (Vec < RepoState > , HashSet < String >) , String > ; }
    };
}

RepoDiscoverer!()