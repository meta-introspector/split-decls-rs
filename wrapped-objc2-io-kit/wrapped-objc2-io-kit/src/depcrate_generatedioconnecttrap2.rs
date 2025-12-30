// Generated macro for IOConnectTrap2 (function)
macro_rules! Depcrate_generatedIOConnectTrap2 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap2"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap2 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap2 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize ,) -> libc :: kern_return_t ; } unsafe { IOConnectTrap2 (connect , index , p1 , p2) } }
};
}
