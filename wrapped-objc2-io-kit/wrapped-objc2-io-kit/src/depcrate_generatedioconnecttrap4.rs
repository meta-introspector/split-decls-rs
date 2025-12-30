// Generated macro for IOConnectTrap4 (function)
macro_rules! Depcrate_generatedIOConnectTrap4 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap4"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap4 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize , p4 : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap4 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize , p4 : usize ,) -> libc :: kern_return_t ; } unsafe { IOConnectTrap4 (connect , index , p1 , p2 , p3 , p4) } }
};
}
