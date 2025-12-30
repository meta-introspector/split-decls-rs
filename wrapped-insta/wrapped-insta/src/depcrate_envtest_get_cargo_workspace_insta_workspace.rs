// Generated macro for test_get_cargo_workspace_insta_workspace (function)
macro_rules! Depcrate_envtest_get_cargo_workspace_insta_workspace {
() => {
// Module: crate::env
// Provides: {"test_get_cargo_workspace_insta_workspace"}
// Dependencies: {}
# [test] fn test_get_cargo_workspace_insta_workspace () { let workspace = get_cargo_workspace (Workspace :: UseAsIs ("/tmp/insta_workspace_root")) ; assert ! (workspace . ends_with ("insta_workspace_root")) ; }
};
}
