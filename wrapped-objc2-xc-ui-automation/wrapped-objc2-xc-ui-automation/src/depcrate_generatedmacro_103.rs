// Generated macro for macro_103 (macro)
macro_rules! Depcrate_generatedmacro_103 {
() => {
// Module: crate::generated
// Provides: {"macro_103"}
// Dependencies: {}
extern_protocol ! (# [doc = " [Apple's documentation](https://developer.apple.com/documentation/xcuiautomation/xcuielementsnapshot?language=objc)"] pub unsafe trait XCUIElementSnapshot : XCUIElementAttributes + MainThreadOnly { # [unsafe (method (children))] # [unsafe (method_family = none)] fn children (& self) -> Retained < NSArray < ProtocolObject < dyn XCUIElementSnapshot >>>; # [doc = " Returns a hierarchical dictionary representation with standard attributes for the element and all"] # [doc = " of its descendants. The dictionary keys are of type XCUIElementAttributeName. If the value for a given"] # [doc = " attribute is null, the key will not be present, but empty strings may be found in the dictionary."] # [unsafe (method (dictionaryRepresentation))] # [unsafe (method_family = none)] fn dictionaryRepresentation (& self ,) -> Retained < NSDictionary < XCUIElementAttributeName , AnyObject >>; }) ;
};
}
