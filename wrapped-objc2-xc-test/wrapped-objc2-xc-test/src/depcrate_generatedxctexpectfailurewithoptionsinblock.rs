// Generated macro for XCTExpectFailureWithOptionsInBlock (function)
macro_rules! Depcrate_generatedXCTExpectFailureWithOptionsInBlock {
() => {
// Module: crate::generated
// Provides: {"XCTExpectFailureWithOptionsInBlock"}
// Dependencies: {}
# [doc = " Like XCTExpectFailure, but takes an options object that can be used to customize the behavior and"] # [doc = " limits the scope in which issues are matched."] # [cfg (feature = "block2")] # [inline] pub extern "C-unwind" fn XCTExpectFailureWithOptionsInBlock (failure_reason : Option < & NSString > , options : & XCTExpectedFailureOptions , failing_block : & block2 :: DynBlock < dyn Fn () > ,) { extern "C-unwind" { fn XCTExpectFailureWithOptionsInBlock (failure_reason : Option < & NSString > , options : & XCTExpectedFailureOptions , failing_block : & block2 :: DynBlock < dyn Fn () > ,) ; } unsafe { XCTExpectFailureWithOptionsInBlock (failure_reason , options , failing_block) } }
};
}
