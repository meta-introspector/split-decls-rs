// Generated macro for IOConnectTrap3 (function)
macro_rules! Depcrate_generatedIOConnectTrap3 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap3"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap3 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap3 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize ,) -> libc :: kern_return_t ; } unsafe { IOConnectTrap3 (connect , index , p1 , p2 , p3) } }
};
}
