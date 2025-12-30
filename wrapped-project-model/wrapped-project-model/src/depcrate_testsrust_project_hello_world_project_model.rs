// Generated macro for rust_project_hello_world_project_model (function)
macro_rules! Depcrate_testsrust_project_hello_world_project_model {
() => {
// Module: crate::tests
// Provides: {"rust_project_hello_world_project_model"}
// Dependencies: {}
# [test] fn rust_project_hello_world_project_model () { let (crate_graph , _proc_macros) = load_rust_project ("hello-world-project.json") ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/rust_project_hello_world_project_model.txt"] ,) ; }
};
}
