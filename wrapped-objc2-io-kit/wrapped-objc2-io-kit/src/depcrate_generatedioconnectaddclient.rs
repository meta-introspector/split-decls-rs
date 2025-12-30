// Generated macro for IOConnectAddClient (function)
macro_rules! Depcrate_generatedIOConnectAddClient {
() => {
// Module: crate::generated
// Provides: {"IOConnectAddClient"}
// Dependencies: {}
# [doc = " Inform a connection of a second connection."] # [doc = ""] # [doc = " This is a generic method to inform a family connection of a second connection, and is rarely used."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Parameter `client`: Another connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Returns: A kern_return_t error code returned by the family."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectAddClient (connect : io_connect_t , client : io_connect_t ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectAddClient (connect : io_connect_t , client : io_connect_t) -> libc :: kern_return_t ; } unsafe { IOConnectAddClient (connect , client) } }
};
}
