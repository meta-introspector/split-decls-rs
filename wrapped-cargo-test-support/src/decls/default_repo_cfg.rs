macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! default_repo_cfg {
    () => {
        deps!();
        fn default_repo_cfg (repo : & git2 :: Repository) { let mut cfg = t ! (repo . config ()) ; t ! (cfg . set_str ("user.email" , "foo@bar.com")) ; t ! (cfg . set_str ("user.name" , "Foo Bar")) ; }
    };
}

default_repo_cfg!()