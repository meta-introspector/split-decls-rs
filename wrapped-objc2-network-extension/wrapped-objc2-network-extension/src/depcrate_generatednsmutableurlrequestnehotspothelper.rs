// Generated macro for NSMutableURLRequestNEHotspotHelper (trait)
macro_rules! Depcrate_generatedNSMutableURLRequestNEHotspotHelper {
() => {
// Module: crate::generated
// Provides: {"NSMutableURLRequestNEHotspotHelper"}
// Dependencies: {}
# [doc = " Category \"NEHotspotHelper\" on [`NSMutableURLRequest`]."] # [doc = ""] # [doc = " Extend NSMutableURLRequest to include a method to bind the"] # [doc = " request to the network interface associated with the specified"] # [doc = " NEHotspotHelperCommand object."] # [doc (alias = "NEHotspotHelper")] pub unsafe trait NSMutableURLRequestNEHotspotHelper : ClassType + Sized + private_NSMutableURLRequestNEHotspotHelper :: Sealed { extern_methods ! (# [doc = " Binds the NSMutableURLRequest to the network interface associated with"] # [doc = " the NEHotspotHelperCommand object."] # [unsafe (method (bindToHotspotHelperCommand :))] # [unsafe (method_family = none)] unsafe fn bindToHotspotHelperCommand (& self , command : & NEHotspotHelperCommand) ;) ; }
};
}
