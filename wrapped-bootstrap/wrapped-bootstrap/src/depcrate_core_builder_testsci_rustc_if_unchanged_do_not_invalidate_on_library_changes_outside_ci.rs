// Generated macro for ci_rustc_if_unchanged_do_not_invalidate_on_library_changes_outside_ci (function)
macro_rules! Depcrate_core_builder_testsci_rustc_if_unchanged_do_not_invalidate_on_library_changes_outside_ci {
() => {
// Module: crate::core::builder::tests
// Provides: {"ci_rustc_if_unchanged_do_not_invalidate_on_library_changes_outside_ci"}
// Dependencies: {}
# [test] fn ci_rustc_if_unchanged_do_not_invalidate_on_library_changes_outside_ci () { git_test (| ctx | { prepare_rustc_checkout (ctx) ; let sha = ctx . create_upstream_merge (& ["compiler/bar"]) ; ctx . create_nonupstream_merge (& ["library/foo"]) ; let config = parse_config_download_rustc_at (ctx . get_path () , "if-unchanged" , false) ; assert_eq ! (config . download_rustc_commit , Some (sha)) ; }) ; }
};
}
