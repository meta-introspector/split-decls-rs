// Generated macro for print_errors (function)
macro_rules! Depcrate_validation_test_harnessprint_errors {
() => {
// Module: crate::validation::test_harness
// Provides: {"print_errors"}
// Dependencies: {}
fn print_errors (errs : & [RuleError]) { for err in errs { for p in err . locations () { print ! ("[{:>3},{:>3},{:>3}]  " , p . index () , p . line () , p . column ()) ; } println ! ("{}" , err . message ()) ; } }
};
}
