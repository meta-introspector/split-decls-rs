// Generated macro for impl_221 (impl)
macro_rules! Depcrate_envimpl_221 {
() => {
// Module: crate::env
// Provides: {"impl_221"}
// Dependencies: {}
# [cfg (feature = "_cargo_insta_internal")] impl ToolConfig { # [doc = " Returns the intended test runner"] pub fn test_runner (& self) -> TestRunner { self . test_runner } # [doc = " Whether to fallback to `cargo test` if the test runner isn't available"] pub fn test_runner_fallback (& self) -> bool { self . test_runner_fallback } pub fn test_unreferenced (& self) -> UnreferencedSnapshots { self . test_unreferenced } # [doc = " Returns the auto review flag."] pub fn auto_review (& self) -> bool { self . auto_review } # [doc = " Returns the auto accept unseen flag."] pub fn auto_accept_unseen (& self) -> bool { self . auto_accept_unseen } pub fn review_include_hidden (& self) -> bool { self . review_include_hidden } pub fn review_include_ignored (& self) -> bool { self . review_include_ignored } pub fn review_warn_undiscovered (& self) -> bool { self . review_warn_undiscovered } }
};
}
