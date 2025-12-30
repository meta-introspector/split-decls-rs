// Generated macro for global_setting (function)
macro_rules! Depcrate_builder_testsglobal_setting {
() => {
// Module: crate::builder::tests
// Provides: {"global_setting"}
// Dependencies: {}
# [test] fn global_setting () { let mut cmd = Command :: new ("test") . disable_version_flag (true) . subcommand (Command :: new ("subcmd")) ; cmd . _propagate () ; assert ! (cmd . get_subcommands () . find (| s | s . get_name () == "subcmd") . unwrap () . is_disable_version_flag_set ()) ; }
};
}
