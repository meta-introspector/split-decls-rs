macro_rules! deps {
    () => {
        DummyCargoTomlProcessor!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (not (feature = "toml_edit_enabled"))] impl CargoTomlProcessor for DummyCargoTomlProcessor { fn process_cargo_toml (& self , _path : & Path , _discovered_repos : & mut HashSet < RepoState > , _all_vendored_crate_names : & mut HashSet < String > , _target_org : & str , _target_branch : & str ,) -> std :: result :: Result < () , String > { Err ("toml_edit_enabled feature is required for CargoTomlProcessor." . to_string ()) } }
    };
}

impl_38!()