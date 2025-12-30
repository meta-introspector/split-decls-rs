// Generated macro for IOConnectAddRef (function)
macro_rules! Depcrate_generatedIOConnectAddRef {
() => {
// Module: crate::generated
// Provides: {"IOConnectAddRef"}
// Dependencies: {}
# [doc = " Adds a reference to the connect handle."] # [doc = ""] # [doc = " Adds a reference to the connect handle."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectAddRef (connect : io_connect_t) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectAddRef (connect : io_connect_t) -> libc :: kern_return_t ; } unsafe { IOConnectAddRef (connect) } }
};
}
