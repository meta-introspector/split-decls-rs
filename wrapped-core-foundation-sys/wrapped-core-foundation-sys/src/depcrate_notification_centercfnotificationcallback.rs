// Generated macro for CFNotificationCallback (type)
macro_rules! Depcrate_notification_centerCFNotificationCallback {
() => {
// Module: crate::notification_center
// Provides: {"CFNotificationCallback"}
// Dependencies: {}
pub type CFNotificationCallback = extern "C" fn (center : CFNotificationCenterRef , observer : * mut c_void , name : CFNotificationName , object : * const c_void , userInfo : CFDictionaryRef ,) ;
};
}
