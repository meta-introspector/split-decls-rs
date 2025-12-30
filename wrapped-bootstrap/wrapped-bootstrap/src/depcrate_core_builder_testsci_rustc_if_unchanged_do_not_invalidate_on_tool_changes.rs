// Generated macro for ci_rustc_if_unchanged_do_not_invalidate_on_tool_changes (function)
macro_rules! Depcrate_core_builder_testsci_rustc_if_unchanged_do_not_invalidate_on_tool_changes {
() => {
// Module: crate::core::builder::tests
// Provides: {"ci_rustc_if_unchanged_do_not_invalidate_on_tool_changes"}
// Dependencies: {}
# [test] fn ci_rustc_if_unchanged_do_not_invalidate_on_tool_changes () { git_test (| ctx | { prepare_rustc_checkout (ctx) ; let sha = ctx . create_upstream_merge (& ["compiler/bar"]) ; ctx . create_nonupstream_merge (& ["src/tools/foo"]) ; let config = parse_config_download_rustc_at (ctx . get_path () , "if-unchanged" , true) ; assert_eq ! (config . download_rustc_commit , Some (sha)) ; }) ; }
};
}
