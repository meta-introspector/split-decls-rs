// Generated macro for order_of_clippy_rules (function)
macro_rules! Depcrate_core_config_testsorder_of_clippy_rules {
() => {
// Module: crate::core::config::tests
// Provides: {"order_of_clippy_rules"}
// Dependencies: {}
# [test] fn order_of_clippy_rules () { let args = vec ! ["clippy" . to_string () , "--fix" . to_string () , "--allow-dirty" . to_string () , "--allow-staged" . to_string () , "-Aclippy:all" . to_string () , "-Wclippy::style" . to_string () , "-Aclippy::foo1" . to_string () , "-Aclippy::foo2" . to_string () ,] ; let config = Config :: parse (Flags :: parse (& args)) ; let actual = match config . cmd . clone () { crate :: Subcommand :: Clippy { allow , deny , warn , forbid , .. } => { let cfg = LintConfig { allow , deny , warn , forbid } ; get_clippy_rules_in_order (& args , & cfg) } _ => panic ! ("invalid subcommand") , } ; let expected = vec ! ["-Aclippy:all" . to_string () , "-Wclippy::style" . to_string () , "-Aclippy::foo1" . to_string () , "-Aclippy::foo2" . to_string () ,] ; assert_eq ! (expected , actual) ; }
};
}
