// Generated macro for impl_145 (impl)
macro_rules! Depcrate_generatedimpl_145 {
() => {
// Module: crate::generated
// Provides: {"impl_145"}
// Dependencies: {}
impl XCTIssueType { # [doc = " Issue raised by a failed XCTAssert or related API."] # [doc (alias = "XCTIssueTypeAssertionFailure")] pub const AssertionFailure : Self = Self (0) ; # [doc = " Issue raised by the test throwing an error in Swift. This could also occur if an Objective C test is implemented in the form `- (BOOL)testFoo:(NSError **)outError` and returns NO with a non-nil out error."] # [doc (alias = "XCTIssueTypeThrownError")] pub const ThrownError : Self = Self (1) ; # [doc = " Code in the test throws and does not catch an exception, Objective C, C++, or other."] # [doc (alias = "XCTIssueTypeUncaughtException")] pub const UncaughtException : Self = Self (2) ; # [doc = " One of the XCTestCase(measure:) family of APIs detected a performance regression."] # [doc (alias = "XCTIssueTypePerformanceRegression")] pub const PerformanceRegression : Self = Self (3) ; # [doc = " One of the framework APIs failed internally. For example, XCUIApplication was unable to launch or terminate an app or XCUIElementQuery was unable to complete a query."] # [doc (alias = "XCTIssueTypeSystem")] pub const System : Self = Self (4) ; # [doc = " Issue raised when XCTExpectFailure is used but no matching issue is recorded."] # [doc (alias = "XCTIssueTypeUnmatchedExpectedFailure")] pub const UnmatchedExpectedFailure : Self = Self (5) ; }
};
}
