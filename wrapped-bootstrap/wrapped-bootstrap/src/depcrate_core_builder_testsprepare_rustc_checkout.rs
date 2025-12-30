// Generated macro for prepare_rustc_checkout (function)
macro_rules! Depcrate_core_builder_testsprepare_rustc_checkout {
() => {
// Module: crate::core::builder::tests
// Provides: {"prepare_rustc_checkout"}
// Dependencies: {}
# [doc = " Prepares the given directory so that it looks like a rustc checkout."] # [doc = " Also configures `GitCtx` to use the correct merge bot e-mail for upstream merge commits."] fn prepare_rustc_checkout (ctx : & mut GitCtx) { ctx . merge_bot_email = format ! ("Merge bot <{}>" , parse_stage0_file () . config . git_merge_commit_email) ; ctx . write ("src/ci/channel" , "nightly") ; ctx . commit () ; }
};
}
