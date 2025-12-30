// Generated macro for macro_104 (macro)
macro_rules! Depcrate_generatedmacro_104 {
() => {
// Module: crate::generated
// Provides: {"macro_104"}
// Dependencies: {}
extern_protocol ! (# [doc = " [Apple's documentation](https://developer.apple.com/documentation/xcuiautomation/xcuielementsnapshotproviding?language=objc)"] pub unsafe trait XCUIElementSnapshotProviding : NSObjectProtocol + MainThreadOnly { # [doc = " Returns a hierarchical data structure with standard attributes for the element and its children."] # [unsafe (method (snapshotWithError : _))] # [unsafe (method_family = none)] fn snapshotWithError (& self ,) -> Result < Retained < ProtocolObject < dyn XCUIElementSnapshot >>, Retained < NSError >>; }) ;
};
}
