// Generated macro for NEHotspotHelperResult (struct)
macro_rules! Depcrate_generatedNEHotspotHelperResult {
() => {
// Module: crate::generated
// Provides: {"NEHotspotHelperResult"}
// Dependencies: {}
# [doc = " The result of processing the NEHotspotHelperCommand."] # [doc = ""] # [doc = " The HotspotHelper provides the result of"] # [doc = " processing the NEHotspotHelperCommand when it instantiates"] # [doc = " its NEHotspotHelperResponse."] # [doc = ""] # [doc = ""] # [doc = ""] # [doc = " interaction. This result is only valid in response to a command with type"] # [doc = " kNEHotspotHelperCommandTypeAuthenticate."] # [doc = ""] # [doc = " recognize the command type."] # [doc = ""] # [doc = " authentication again. This result is only valid in response to a"] # [doc = " command with type kNEHotspotHelperCommandTypeMaintain."] # [doc = ""] # [doc = " authenticate, the helper determined that it can't perform the"] # [doc = " authentication. This result is only valid in response to commands of type"] # [doc = " kNEHotspotHelperCommandTypeAuthenticate and"] # [doc = " kNEHotspotHelperCommandTypePresentUI."] # [doc = ""] # [doc = " it is temporarily unable to perform the authentication."] # [doc = " This result is only valid in response to commands of type"] # [doc = " kNEHotspotHelperCommandTypeAuthenticate and"] # [doc = " kNEHotspotHelperCommandTypePresentUI."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/networkextension/nehotspothelperresult?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct NEHotspotHelperResult (pub NSInteger) ;
};
}
