// Generated macro for XCKeyValueObservingExpectationHandler (type)
macro_rules! Depcrate_generatedXCKeyValueObservingExpectationHandler {
() => {
// Module: crate::generated
// Provides: {"XCKeyValueObservingExpectationHandler"}
// Dependencies: {}
# [doc = " A block to be invoked when a change is observed for the keyPath of the observed object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `observedObject`: The observed object, provided to avoid block capture issues."] # [doc = ""] # [doc = ""] # [doc = " Parameter `change`: The KVO change dictionary."] # [doc = ""] # [doc = ""] # [doc = " Returns: Return YES if the expectation is fulfilled, NO if it is not."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xckeyvalueobservingexpectationhandler?language=objc)"] # [cfg (feature = "block2")] pub type XCKeyValueObservingExpectationHandler = * mut block2 :: DynBlock < dyn Fn (NonNull < AnyObject > , NonNull < NSDictionary >) -> Bool > ;
};
}
