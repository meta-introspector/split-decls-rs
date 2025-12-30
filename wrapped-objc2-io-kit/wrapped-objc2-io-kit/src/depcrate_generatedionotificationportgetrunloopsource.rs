// Generated macro for IONotificationPortGetRunLoopSource (function)
macro_rules! Depcrate_generatedIONotificationPortGetRunLoopSource {
() => {
// Module: crate::generated
// Provides: {"IONotificationPortGetRunLoopSource"}
// Dependencies: {}
# [deprecated = "renamed to `IONotificationPort::run_loop_source`"] # [inline] pub unsafe extern "C-unwind" fn IONotificationPortGetRunLoopSource (notify : IONotificationPortRef ,) -> Option < CFRetained < CFRunLoopSource > > { extern "C-unwind" { fn IONotificationPortGetRunLoopSource (notify : IONotificationPortRef ,) -> Option < NonNull < CFRunLoopSource > > ; } let ret = unsafe { IONotificationPortGetRunLoopSource (notify) } ; ret . map (| ret | unsafe { CFRetained :: retain (ret) }) }
};
}
