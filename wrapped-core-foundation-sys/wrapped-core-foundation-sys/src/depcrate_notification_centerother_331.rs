// Generated macro for other_331 (other)
macro_rules! Depcrate_notification_centerother_331 {
() => {
// Module: crate::notification_center
// Provides: {"other_331"}
// Dependencies: {}
unsafe extern "C" { pub fn CFNotificationCenterGetDarwinNotifyCenter () -> CFNotificationCenterRef ; # [cfg (any (target_os = "macos" , target_os = "windows"))] pub fn CFNotificationCenterGetDistributedCenter () -> CFNotificationCenterRef ; pub fn CFNotificationCenterGetLocalCenter () -> CFNotificationCenterRef ; pub fn CFNotificationCenterPostNotification (center : CFNotificationCenterRef , name : CFNotificationName , object : * const c_void , userInfo : CFDictionaryRef , deliverImmediately : Boolean ,) ; pub fn CFNotificationCenterPostNotificationWithOptions (center : CFNotificationCenterRef , name : CFNotificationName , object : * const c_void , userInfo : CFDictionaryRef , options : CFOptionFlags ,) ; pub fn CFNotificationCenterAddObserver (center : CFNotificationCenterRef , observer : * const c_void , callBack : CFNotificationCallback , name : CFStringRef , object : * const c_void , suspensionBehavior : CFNotificationSuspensionBehavior ,) ; pub fn CFNotificationCenterRemoveEveryObserver (center : CFNotificationCenterRef , observer : * const c_void ,) ; pub fn CFNotificationCenterRemoveObserver (center : CFNotificationCenterRef , observer : * const c_void , name : CFNotificationName , object : * const c_void ,) ; pub fn CFNotificationCenterGetTypeID () -> CFTypeID ; }
};
}
