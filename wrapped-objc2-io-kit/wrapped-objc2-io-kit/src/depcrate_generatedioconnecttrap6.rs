// Generated macro for IOConnectTrap6 (function)
macro_rules! Depcrate_generatedIOConnectTrap6 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap6"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap6 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize , p4 : usize , p5 : usize , p6 : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap6 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize , p4 : usize , p5 : usize , p6 : usize ,) -> libc :: kern_return_t ; } unsafe { IOConnectTrap6 (connect , index , p1 , p2 , p3 , p4 , p5 , p6) } }
};
}
