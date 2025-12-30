// Generated macro for IOServiceClose (function)
macro_rules! Depcrate_generatedIOServiceClose {
() => {
// Module: crate::generated
// Provides: {"IOServiceClose"}
// Dependencies: {}
# [doc = " Close a connection to an IOService and destroy the connect handle."] # [doc = ""] # [doc = " A connection created with the IOServiceOpen should be closed when the connection is no longer to be used with IOServiceClose."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen. It will be destroyed by this function, and should not be released with IOObjectRelease."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOServiceClose (connect : io_connect_t) -> libc :: kern_return_t { extern "C-unwind" { fn IOServiceClose (connect : io_connect_t) -> libc :: kern_return_t ; } unsafe { IOServiceClose (connect) } }
};
}
