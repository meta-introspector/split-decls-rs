// Generated macro for ToolConfig (struct)
macro_rules! Depcrate_envToolConfig {
() => {
// Module: crate::env
// Provides: {"ToolConfig"}
// Dependencies: {}
# [doc = " Represents a tool configuration."] # [derive (Debug , Clone)] pub struct ToolConfig { force_pass : bool , require_full_match : bool , output : OutputBehavior , snapshot_update : SnapshotUpdate , # [cfg (feature = "glob")] glob_fail_fast : bool , # [cfg (feature = "_cargo_insta_internal")] test_runner_fallback : bool , # [cfg (feature = "_cargo_insta_internal")] test_runner : TestRunner , # [cfg (feature = "_cargo_insta_internal")] test_unreferenced : UnreferencedSnapshots , # [cfg (feature = "_cargo_insta_internal")] auto_review : bool , # [cfg (feature = "_cargo_insta_internal")] auto_accept_unseen : bool , # [cfg (feature = "_cargo_insta_internal")] review_include_ignored : bool , # [cfg (feature = "_cargo_insta_internal")] review_include_hidden : bool , # [cfg (feature = "_cargo_insta_internal")] review_warn_undiscovered : bool , }
};
}
