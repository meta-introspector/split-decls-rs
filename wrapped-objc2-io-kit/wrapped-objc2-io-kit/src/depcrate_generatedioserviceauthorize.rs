// Generated macro for IOServiceAuthorize (function)
macro_rules! Depcrate_generatedIOServiceAuthorize {
() => {
// Module: crate::generated
// Provides: {"IOServiceAuthorize"}
// Dependencies: {}
# [doc = " Authorize access to an IOService."] # [doc = ""] # [doc = " Determine whether this application is authorized to invoke IOServiceOpen() for a given IOService, either by confirming that it has been previously authorized by the user, or by soliciting the console user."] # [doc = ""] # [doc = " Parameter `service`: The IOService object to be authorized, usually obtained via the IOServiceGetMatchingServices or IOServiceAddNotification APIs."] # [doc = ""] # [doc = " Parameter `options`: kIOServiceInteractionAllowed may be set to permit user interaction, if required."] # [doc = ""] # [doc = " Returns: kIOReturnSuccess if the IOService is authorized, kIOReturnNotPermitted if the IOService is not authorized."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOServiceAuthorize (service : io_service_t , options : u32 ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOServiceAuthorize (service : io_service_t , options : u32) -> libc :: kern_return_t ; } unsafe { IOServiceAuthorize (service , options) } }
};
}
