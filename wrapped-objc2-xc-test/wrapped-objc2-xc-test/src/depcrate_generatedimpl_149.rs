// Generated macro for impl_149 (impl)
macro_rules! Depcrate_generatedimpl_149 {
() => {
// Module: crate::generated
// Provides: {"impl_149"}
// Dependencies: {}
impl XCTIssueSeverity { # [doc = " The severity level for an issue which should be noted but is not"] # [doc = " necessarily an error."] # [doc = ""] # [doc = " An issue with warning severity does not cause the test it's associated"] # [doc = " with to be marked as a failure, but is noted in the results."] # [doc (alias = "XCTIssueSeverityWarning")] pub const Warning : Self = Self (4) ; # [doc = " The severity level for an issue which represents an error in a test."] # [doc = ""] # [doc = " An issue with error severity causes the test it's associated with to be"] # [doc = " marked as a failure."] # [doc (alias = "XCTIssueSeverityError")] pub const Error : Self = Self (8) ; }
};
}
