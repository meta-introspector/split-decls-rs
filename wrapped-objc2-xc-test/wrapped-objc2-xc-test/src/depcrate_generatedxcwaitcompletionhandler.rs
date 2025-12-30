// Generated macro for XCWaitCompletionHandler (type)
macro_rules! Depcrate_generatedXCWaitCompletionHandler {
() => {
// Module: crate::generated
// Provides: {"XCWaitCompletionHandler"}
// Dependencies: {}
# [doc = " A block to be invoked when a call to -waitForExpectationsWithTimeout:handler: times out or has"] # [doc = " had all associated expectations fulfilled."] # [doc = ""] # [doc = ""] # [doc = " Parameter `error`: If the wait timed out or a failure was raised while waiting, the error's code"] # [doc = " will specify the type of failure. Otherwise error will be nil."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xcwaitcompletionhandler?language=objc)"] # [cfg (feature = "block2")] pub type XCWaitCompletionHandler = * mut block2 :: DynBlock < dyn Fn (* mut NSError) > ;
};
}
