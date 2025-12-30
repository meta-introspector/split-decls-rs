// Generated macro for __kuser_cmpxchg (function)
macro_rules! Depcrate_arm_linux__kuser_cmpxchg {
() => {
// Module: crate::arm_linux
// Provides: {"__kuser_cmpxchg"}
// Dependencies: {}
unsafe fn __kuser_cmpxchg (oldval : u32 , newval : u32 , ptr : * mut u32) -> bool { let f = unsafe { mem :: transmute :: < _ , extern "C" fn (u32 , u32 , * mut u32) -> u32 > (0xffff0fc0usize as * const ()) } ; f (oldval , newval , ptr) == 0 }
};
}
