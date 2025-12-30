// Generated macro for IOConnectTrap5 (function)
macro_rules! Depcrate_generatedIOConnectTrap5 {
() => {
// Module: crate::generated
// Provides: {"IOConnectTrap5"}
// Dependencies: {}
# [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectTrap5 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize , p4 : usize , p5 : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectTrap5 (connect : io_connect_t , index : u32 , p1 : usize , p2 : usize , p3 : usize , p4 : usize , p5 : usize ,) -> libc :: kern_return_t ; } unsafe { IOConnectTrap5 (connect , index , p1 , p2 , p3 , p4 , p5) } }
};
}
