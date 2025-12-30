// Generated macro for issue_2090 (function)
macro_rules! Depcrate_builder_testsissue_2090 {
() => {
// Module: crate::builder::tests
// Provides: {"issue_2090"}
// Dependencies: {}
# [test] fn issue_2090 () { let mut cmd = Command :: new ("cmd") . disable_version_flag (true) . subcommand (Command :: new ("sub")) ; cmd . _build_self (false) ; assert ! (cmd . get_subcommands () . next () . unwrap () . is_disable_version_flag_set ()) ; }
};
}
