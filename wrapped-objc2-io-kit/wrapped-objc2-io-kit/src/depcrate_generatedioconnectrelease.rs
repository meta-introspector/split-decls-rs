// Generated macro for IOConnectRelease (function)
macro_rules! Depcrate_generatedIOConnectRelease {
() => {
// Module: crate::generated
// Provides: {"IOConnectRelease"}
// Dependencies: {}
# [doc = " Remove a reference to the connect handle."] # [doc = ""] # [doc = " Removes a reference to the connect handle.  If the last reference is removed an implicit IOServiceClose is performed."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectRelease (connect : io_connect_t) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectRelease (connect : io_connect_t) -> libc :: kern_return_t ; } unsafe { IOConnectRelease (connect) } }
};
}
