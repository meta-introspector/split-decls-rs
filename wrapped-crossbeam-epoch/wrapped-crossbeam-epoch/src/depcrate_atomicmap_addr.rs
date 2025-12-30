// Generated macro for map_addr (function)
macro_rules! Depcrate_atomicmap_addr {
() => {
// Module: crate::atomic
// Provides: {"map_addr"}
// Dependencies: {}
# [inline] fn map_addr < T > (ptr : * mut T , f : impl FnOnce (usize) -> usize) -> * mut T { let new_addr = f (ptr as usize) ; ptr . cast :: < u8 > () . wrapping_add (new_addr . wrapping_sub (ptr as usize)) . cast :: < T > () }
};
}
