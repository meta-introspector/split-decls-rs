// Generated macro for optgroups (function)
macro_rules! Depcrate_testsoptgroups {
() => {
// Module: crate::tests
// Provides: {"optgroups"}
// Dependencies: {}
fn optgroups () -> getopts :: Options { let mut opts = getopts :: Options :: new () ; for group in rustc_optgroups () { group . apply (& mut opts) ; } return opts ; }
};
}
