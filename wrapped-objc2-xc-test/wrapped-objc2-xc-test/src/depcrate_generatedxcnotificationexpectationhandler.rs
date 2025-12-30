// Generated macro for XCNotificationExpectationHandler (type)
macro_rules! Depcrate_generatedXCNotificationExpectationHandler {
() => {
// Module: crate::generated
// Provides: {"XCNotificationExpectationHandler"}
// Dependencies: {}
# [doc = " A block to be invoked when a notification matching the specified name is observed"] # [doc = " from the object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `notification`: The notification object."] # [doc = ""] # [doc = ""] # [doc = " Returns: Return YES if the expectation is fulfilled, NO if it is not."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xcnotificationexpectationhandler?language=objc)"] # [cfg (feature = "block2")] pub type XCNotificationExpectationHandler = * mut block2 :: DynBlock < dyn Fn (NonNull < NSNotification >) -> Bool > ;
};
}
