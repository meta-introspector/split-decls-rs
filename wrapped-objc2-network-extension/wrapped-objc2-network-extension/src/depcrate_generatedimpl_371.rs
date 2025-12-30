// Generated macro for impl_371 (impl)
macro_rules! Depcrate_generatedimpl_371 {
() => {
// Module: crate::generated
// Provides: {"impl_371"}
// Dependencies: {}
impl NEHotspotHelperResponse { extern_methods ! (# [doc = " Set the network that conveys the confidence level."] # [doc = ""] # [doc = " Provide the annotated NEHotspotNetwork object in the response to the"] # [doc = " kNEHotspotHelperCommandTypeEvaluate command. The helper sets the"] # [doc = " confidence in the network object to indicate its ability to handle"] # [doc = " the current network."] # [unsafe (method (setNetwork :))] # [unsafe (method_family = none)] pub unsafe fn setNetwork (& self , network : & NEHotspotNetwork) ; # [doc = " Set the list of handled networks."] # [doc = ""] # [doc = " Provide an NSArray of annotated NEHotspotNetwork objects in response"] # [doc = " to the kNEHotspotHelperCommandTypeFilterScanList command."] # [doc = " The helper provides the list of network objects that it is capable of"] # [doc = " handling with at least low confidence. Networks that it has no"] # [doc = " confidence in handling should not be specified."] # [unsafe (method (setNetworkList :))] # [unsafe (method_family = none)] pub unsafe fn setNetworkList (& self , network_list : & NSArray < NEHotspotNetwork >) ; # [doc = " Delivers the response to the command."] # [doc = ""] # [doc = " Deliver the NEHotspotHelperResponse to the HotspotHelper infrastructure."] # [deprecated = "Use handleCommand in NEHotspotEvaluationProvider or NEHotspotAuthenticationProvider API"] # [unsafe (method (deliver))] # [unsafe (method_family = none)] pub unsafe fn deliver (& self) ;) ; }
};
}
