// Generated macro for git_test (function)
macro_rules! Depcrate_utils_build_stamp_tests_gitgit_test {
() => {
// Module: crate::utils::build_stamp::tests::git
// Provides: {"git_test"}
// Dependencies: {}
# [doc = " Run an end-to-end test that allows testing git logic."] pub fn git_test < F > (test_fn : F) where F : FnOnce (& mut GitCtx) , { let mut ctx = GitCtx :: new () ; test_fn (& mut ctx) ; }
};
}
