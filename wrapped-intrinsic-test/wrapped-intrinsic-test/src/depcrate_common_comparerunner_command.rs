// Generated macro for runner_command (function)
macro_rules! Depcrate_common_comparerunner_command {
() => {
// Module: crate::common::compare
// Provides: {"runner_command"}
// Dependencies: {}
fn runner_command (runner : & str) -> Command { let mut it = runner . split_whitespace () ; let mut cmd = Command :: new (it . next () . unwrap ()) ; cmd . args (it) ; cmd }
};
}
