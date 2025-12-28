macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! append_config_to_repo_config {
    () => {
        deps!();
        pub fn append_config_to_repo_config (repo : & mut Repository , config : gix_config :: File < 'static >) { let repo_config = gix_features :: threading :: OwnShared :: make_mut (& mut repo . config . resolved) ; repo_config . append (config) ; }
    };
}

append_config_to_repo_config!();