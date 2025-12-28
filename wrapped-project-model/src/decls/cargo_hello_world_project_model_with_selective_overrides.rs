macro_rules! deps {
    () => {
        CfgOverrides!();
    };
}

macro_rules! cargo_hello_world_project_model_with_selective_overrides {
    () => {
        deps!();
        # [test] fn cargo_hello_world_project_model_with_selective_overrides () { let cfg_overrides = CfgOverrides { global : Default :: default () , selective : std :: iter :: once (("libc" . to_owned () , CfgDiff :: new (Vec :: new () , vec ! [CfgAtom :: Flag (sym :: test)]) ,)) . collect () , } ; let (crate_graph , _proc_macros) = load_cargo_with_overrides ("hello-world-metadata.json" , cfg_overrides) ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/cargo_hello_world_project_model_with_selective_overrides.txt"] ,) }
    };
}

cargo_hello_world_project_model_with_selective_overrides!();