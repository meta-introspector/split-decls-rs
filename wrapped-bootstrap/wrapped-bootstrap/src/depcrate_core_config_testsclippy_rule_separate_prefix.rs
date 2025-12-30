// Generated macro for clippy_rule_separate_prefix (function)
macro_rules! Depcrate_core_config_testsclippy_rule_separate_prefix {
() => {
// Module: crate::core::config::tests
// Provides: {"clippy_rule_separate_prefix"}
// Dependencies: {}
# [test] fn clippy_rule_separate_prefix () { let args = vec ! ["clippy" . to_string () , "-A clippy:all" . to_string () , "-W clippy::style" . to_string ()] ; let config = Config :: parse (Flags :: parse (& args)) ; let actual = match config . cmd . clone () { crate :: Subcommand :: Clippy { allow , deny , warn , forbid , .. } => { let cfg = LintConfig { allow , deny , warn , forbid } ; get_clippy_rules_in_order (& args , & cfg) } _ => panic ! ("invalid subcommand") , } ; let expected = vec ! ["-A clippy:all" . to_string () , "-W clippy::style" . to_string ()] ; assert_eq ! (expected , actual) ; }
};
}
