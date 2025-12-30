// Generated macro for other_4819 (other)
macro_rules! Depcrate_generatedother_4819 {
() => {
// Module: crate::generated
// Provides: {"other_4819"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns the IOService a connect handle was opened on."] # [doc = ""] # [doc = " Finds the service object a connection was opened on."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Parameter `service`: On success, the service handle the connection was opened on, which should be released with IOObjectRelease."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `service` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOConnectGetService (connect : io_connect_t , service : * mut io_service_t ,) -> libc :: kern_return_t ; }
};
}
