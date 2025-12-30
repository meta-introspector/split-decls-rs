// Generated macro for XCTExpectFailureInBlock (function)
macro_rules! Depcrate_generatedXCTExpectFailureInBlock {
() => {
// Module: crate::generated
// Provides: {"XCTExpectFailureInBlock"}
// Dependencies: {}
# [doc = " Like XCTExpectFailure, but limits the scope in which issues are matched."] # [doc = ""] # [doc = ""] # [doc = " Parameter `failingBlock`: The scope of code in which the failure is expected. Note that this will only"] # [doc = " match against failures in that scope on the same thread; failures in dispatch callouts or other code"] # [doc = " running on a different thread will not be matched."] # [cfg (feature = "block2")] # [inline] pub extern "C-unwind" fn XCTExpectFailureInBlock (failure_reason : Option < & NSString > , failing_block : & block2 :: DynBlock < dyn Fn () > ,) { extern "C-unwind" { fn XCTExpectFailureInBlock (failure_reason : Option < & NSString > , failing_block : & block2 :: DynBlock < dyn Fn () > ,) ; } unsafe { XCTExpectFailureInBlock (failure_reason , failing_block) } }
};
}
