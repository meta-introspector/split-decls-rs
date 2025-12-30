// Generated macro for XCTExpectFailureWithOptions (function)
macro_rules! Depcrate_generatedXCTExpectFailureWithOptions {
() => {
// Module: crate::generated
// Provides: {"XCTExpectFailureWithOptions"}
// Dependencies: {}
# [doc = " Like XCTExpectFailure, but takes an options object that can be used to customize the behavior."] # [doc = ""] # [doc = ""] # [doc = " Parameter `options`: The options can include a custom issue matching block as well as the ability to"] # [doc = " disable \"strict\" behavior, which relaxes the requirement that a call to XCTExpectFailure must be matched"] # [doc = " against at least one recorded issue."] # [inline] pub extern "C-unwind" fn XCTExpectFailureWithOptions (failure_reason : Option < & NSString > , options : & XCTExpectedFailureOptions ,) { extern "C-unwind" { fn XCTExpectFailureWithOptions (failure_reason : Option < & NSString > , options : & XCTExpectedFailureOptions ,) ; } unsafe { XCTExpectFailureWithOptions (failure_reason , options) } }
};
}
