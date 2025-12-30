// Generated macro for __kuser_memory_barrier (function)
macro_rules! Depcrate_arm_linux__kuser_memory_barrier {
() => {
// Module: crate::arm_linux
// Provides: {"__kuser_memory_barrier"}
// Dependencies: {}
unsafe fn __kuser_memory_barrier () { let f = unsafe { mem :: transmute :: < _ , extern "C" fn () > (0xffff0fa0usize as * const ()) } ; f () ; }
};
}
