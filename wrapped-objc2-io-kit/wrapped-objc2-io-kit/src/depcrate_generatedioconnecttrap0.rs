// Generated macro for IOConnectTrap0 (function)
macro_rules! Depcrate_generatedIOConnectTrap0 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap0"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap0 (connect : io_connect_t , index : u32) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap0 (connect : io_connect_t , index : u32) -> libc :: kern_return_t ; } unsafe { IOConnectTrap0 (connect , index) } }
};
}
