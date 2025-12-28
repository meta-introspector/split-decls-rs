macro_rules! rust_project_cfg_groups {
    () => {
        # [test] fn rust_project_cfg_groups () { let (crate_graph , _proc_macros) = load_rust_project ("cfg-groups.json") ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/rust_project_cfg_groups.txt"]) ; }
    };
}

rust_project_cfg_groups!();