// Generated macro for XCPredicateExpectationHandler (type)
macro_rules! Depcrate_generatedXCPredicateExpectationHandler {
() => {
// Module: crate::generated
// Provides: {"XCPredicateExpectationHandler"}
// Dependencies: {}
# [doc = " Handler called when evaluating the predicate against the object returns true. If the handler is not"] # [doc = " provided the first successful evaluation will fulfill the expectation. If provided, the handler will"] # [doc = " be queried each time the notification is received to determine whether the expectation should be fulfilled"] # [doc = " or not."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xcpredicateexpectationhandler?language=objc)"] # [cfg (feature = "block2")] pub type XCPredicateExpectationHandler = * mut block2 :: DynBlock < dyn Fn () -> Bool > ;
};
}
