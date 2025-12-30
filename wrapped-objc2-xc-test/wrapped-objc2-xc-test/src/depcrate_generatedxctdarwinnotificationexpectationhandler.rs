// Generated macro for XCTDarwinNotificationExpectationHandler (type)
macro_rules! Depcrate_generatedXCTDarwinNotificationExpectationHandler {
() => {
// Module: crate::generated
// Provides: {"XCTDarwinNotificationExpectationHandler"}
// Dependencies: {}
# [doc = " Handler called when the expectation has received the Darwin notification. If the handler is not"] # [doc = " provided the first posting of the notification will fulfill the expectation. If provided, the handler"] # [doc = " will be queried each time the notification is received to determine whether the expectation should"] # [doc = " be fulfilled or not. This allows the caller to check Darwin state variables or perform other logic"] # [doc = " beyond simply verifying that the notification has been posted."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xctdarwinnotificationexpectationhandler?language=objc)"] # [cfg (feature = "block2")] pub type XCTDarwinNotificationExpectationHandler = * mut block2 :: DynBlock < dyn Fn () -> Bool > ;
};
}
