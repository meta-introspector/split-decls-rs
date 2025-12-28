macro_rules! cargo_hello_world_project_model {
    () => {
        # [test] fn cargo_hello_world_project_model () { let (crate_graph , _proc_macros) = load_cargo ("hello-world-metadata.json") ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/cargo_hello_world_project_model.txt"] ,) }
    };
}

cargo_hello_world_project_model!();