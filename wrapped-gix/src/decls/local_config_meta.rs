macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! local_config_meta {
    () => {
        deps!();
        fn local_config_meta (repo : & Repository) -> gix_config :: file :: Metadata { let meta = repo . config . resolved . meta () . clone () ; assert_eq ! (meta . source , gix_config :: Source :: Local , "local path is the default for new sections") ; meta }
    };
}

local_config_meta!();