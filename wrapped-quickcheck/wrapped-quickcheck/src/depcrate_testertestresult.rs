// Generated macro for TestResult (struct)
macro_rules! Depcrate_testerTestResult {
() => {
// Module: crate::tester
// Provides: {"TestResult"}
// Dependencies: {}
# [doc = " Describes the status of a single instance of a test."] # [doc = ""] # [doc = " All testable things must be capable of producing a `TestResult`."] # [derive (Clone , Debug , PartialEq)] pub struct TestResult { status : Status , arguments : Option < Vec < String > > , err : Option < String > , }
};
}
