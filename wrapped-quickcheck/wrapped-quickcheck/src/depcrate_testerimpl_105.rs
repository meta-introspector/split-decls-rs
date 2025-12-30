// Generated macro for impl_105 (impl)
macro_rules! Depcrate_testerimpl_105 {
() => {
// Module: crate::tester
// Provides: {"impl_105"}
// Dependencies: {}
impl From < bool > for TestResult { # [doc = " A shorter way of producing a `TestResult` from a `bool`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use quickcheck::TestResult;"] # [doc = " let result: TestResult = (2 > 1).into();"] # [doc = " assert_eq!(result, TestResult::passed());"] # [doc = " ```"] fn from (b : bool) -> TestResult { TestResult :: from_bool (b) } }
};
}
