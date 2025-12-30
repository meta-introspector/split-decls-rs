// Generated macro for rust_project_cfg_groups (function)
macro_rules! Depcrate_testsrust_project_cfg_groups {
() => {
// Module: crate::tests
// Provides: {"rust_project_cfg_groups"}
// Dependencies: {}
# [test] fn rust_project_cfg_groups () { let (crate_graph , _proc_macros) = load_rust_project ("cfg-groups.json") ; check_crate_graph (crate_graph , expect_file ! ["../test_data/output/rust_project_cfg_groups.txt"]) ; }
};
}
