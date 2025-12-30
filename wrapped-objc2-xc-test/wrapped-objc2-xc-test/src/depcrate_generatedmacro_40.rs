// Generated macro for macro_40 (macro)
macro_rules! Depcrate_generatedmacro_40 {
() => {
// Module: crate::generated
// Provides: {"macro_40"}
// Dependencies: {}
extern_protocol ! (# [doc = " Represents a test activity."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/xctest/xctactivity?language=objc)"] pub unsafe trait XCTActivity : NSObjectProtocol { # [doc = " Human-readable name of the activity, given at creation time."] # [unsafe (method (name))] # [unsafe (method_family = none)] fn name (& self) -> Retained < NSString >; # [doc = " Adds an attachment which is always kept by Xcode, regardless of the test result."] # [doc = " Thread-safe, attachments can be added from any thread, are reported in the order they are added."] # [unsafe (method (addAttachment :))] # [unsafe (method_family = none)] fn addAttachment (& self , attachment : & XCTAttachment) ; }) ;
};
}
