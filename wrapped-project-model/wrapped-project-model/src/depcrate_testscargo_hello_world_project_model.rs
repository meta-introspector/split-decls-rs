// Generated macro for cargo_hello_world_project_model (function)
macro_rules! Depcrate_testscargo_hello_world_project_model {
() => {
// Module: crate::tests
// Provides: {"cargo_hello_world_project_model"}
// Dependencies: {}
# [test] fn cargo_hello_world_project_model () { let (crate_graph , _proc_macros) = load_cargo ("hello-world-metadata.json") ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/cargo_hello_world_project_model.txt"] ,) }
};
}
