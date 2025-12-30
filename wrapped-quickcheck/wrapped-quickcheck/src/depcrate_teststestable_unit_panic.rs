// Generated macro for testable_unit_panic (function)
macro_rules! Depcrate_teststestable_unit_panic {
() => {
// Module: crate::tests
// Provides: {"testable_unit_panic"}
// Dependencies: {}
# [test] fn testable_unit_panic () { fn panic () { panic ! () ; } assert ! (QuickCheck :: new () . quicktest (panic as fn ()) . is_err ()) ; }
};
}
