// Generated macro for IOConnectTrap1 (function)
macro_rules! Depcrate_generatedIOConnectTrap1 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap1"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap1 (connect : io_connect_t , index : u32 , p1 : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap1 (connect : io_connect_t , index : u32 , p1 : usize) -> libc :: kern_return_t ; } unsafe { IOConnectTrap1 (connect , index , p1) } }
};
}
