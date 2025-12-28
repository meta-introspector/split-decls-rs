macro_rules! rust_project_hello_world_project_model {
    () => {
        # [test] fn rust_project_hello_world_project_model () { let (crate_graph , _proc_macros) = load_rust_project ("hello-world-project.json") ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/rust_project_hello_world_project_model.txt"] ,) ; }
    };
}

rust_project_hello_world_project_model!();