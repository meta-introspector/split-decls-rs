// Generated macro for XCTExpectFailure (function)
macro_rules! Depcrate_generatedXCTExpectFailure {
() => {
// Module: crate::generated
// Provides: {"XCTExpectFailure"}
// Dependencies: {}
# [doc = " Declares that the test is expected to fail at some point beyond the call. This can be used to both document and"] # [doc = " suppress a known issue when immediate resolution is not possible. Issues caught by XCTExpectFailure do not"] # [doc = " impact the aggregate results of the test suites which own them."] # [doc = ""] # [doc = " This function may be invoked repeatedly and has stack semantics. Failures are associated with the closest"] # [doc = " matching expected failure and the stack is cleaned up by the test after it runs. If a failure is expected"] # [doc = " but none is recorded, a distinct failure for the unmatched expected failure will be recorded instead."] # [doc = ""] # [doc = " Threading considerations: when XCTExpectFailure is called on the test's primary thread it will match against"] # [doc = " any issue recorded on any thread. When XCTExpectFailure is called on any other thread, it will only match"] # [doc = " against issues recorded on the same thread."] # [doc = ""] # [doc = ""] # [doc = " Parameter `failureReason`: Explanation of the issue being suppressed. If it contains"] # [doc = " a URL, that URL can be extracted and presented as a link in reporting UI (Xcode and CI)."] # [inline] pub extern "C-unwind" fn XCTExpectFailure (failure_reason : Option < & NSString >) { extern "C-unwind" { fn XCTExpectFailure (failure_reason : Option < & NSString >) ; } unsafe { XCTExpectFailure (failure_reason) } }
};
}
