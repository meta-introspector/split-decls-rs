// Generated macro for macro_83 (macro)
macro_rules! Depcrate_generatedmacro_83 {
() => {
// Module: crate::generated
// Provides: {"macro_83"}
// Dependencies: {}
extern_protocol ! (# [doc = " [Apple's documentation](https://developer.apple.com/documentation/xcuiautomation/xcuiscreenshotproviding?language=objc)"] pub unsafe trait XCUIScreenshotProviding : NSObjectProtocol + MainThreadOnly { # [doc = " Captures and returns a screenshot of the receiver."] # [doc = ""] # [doc = " Equivalent to capturing a screenshot manually, e.g. if two windows are overlapping and"] # [doc = " the occluded window is captured, the front window will be visible in the screenshot."] # [unsafe (method (screenshot))] # [unsafe (method_family = none)] fn screenshot (& self) -> Retained < XCUIScreenshot >; }) ;
};
}
