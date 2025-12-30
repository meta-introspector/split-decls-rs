// Generated macro for ATTrackingManagerAuthorizationStatus (struct)
macro_rules! Depcrate_generatedATTrackingManagerAuthorizationStatus {
() => {
// Module: crate::generated
// Provides: {"ATTrackingManagerAuthorizationStatus"}
// Dependencies: {}
# [doc = " The status values for app tracking authorization."] # [doc = ""] # [doc = " After a device receives an authorization request to approve access to app-related"] # [doc = " data that can be used for tracking the user or the device, the returned value is"] # [doc = " either:"] # [doc = ""] # [doc = " - ``AppTrackingTransparency/ATTrackingManager/AuthorizationStatus/authorized``, or"] # [doc = " - ``AppTrackingTransparency/ATTrackingManager/AuthorizationStatus/denied``."] # [doc = ""] # [doc = " Before a device receives an authorization request to approve access to app-related"] # [doc = " data that can be used for tracking the user or the device, the returned value is:"] # [doc = " ``AppTrackingTransparency/ATTrackingManager/AuthorizationStatus/notDetermined``."] # [doc = ""] # [doc = " If authorization to use app tracking data is restricted, the value is: ``AppTrackingTransparency/ATTrackingManager/AuthorizationStatus/restricted``."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/apptrackingtransparency/attrackingmanagerauthorizationstatus?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct ATTrackingManagerAuthorizationStatus (pub NSUInteger) ;
};
}
