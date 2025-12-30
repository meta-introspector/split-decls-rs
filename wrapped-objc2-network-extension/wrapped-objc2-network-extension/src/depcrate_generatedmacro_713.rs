// Generated macro for macro_713 (macro)
macro_rules! Depcrate_generatedmacro_713 {
() => {
// Module: crate::generated
// Provides: {"macro_713"}
// Dependencies: {}
extern_protocol ! (# [doc = " Delegate for NEAppPushManager."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/networkextension/neapppushdelegate?language=objc)"] pub unsafe trait NEAppPushDelegate : NSObjectProtocol { # [doc = " This delegate method is called when the provider reports incoming call using reportIncomingCommunicationWithUserInfo method."] # [doc = ""] # [doc = " Parameter `userInfo`: A dictionary of custom information that the provider passes to reportIncomingCommunicationWithUserInfo method."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `user_info` generic should be of the correct type."] # [unsafe (method (appPushManager : didReceiveIncomingCallWithUserInfo :))] # [unsafe (method_family = none)] unsafe fn appPushManager_didReceiveIncomingCallWithUserInfo (& self , manager : & NEAppPushManager , user_info : & NSDictionary ,) ; }) ;
};
}
