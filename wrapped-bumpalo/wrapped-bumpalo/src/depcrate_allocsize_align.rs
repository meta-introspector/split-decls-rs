// Generated macro for size_align (function)
macro_rules! Depcrate_allocsize_align {
() => {
// Module: crate::alloc
// Provides: {"size_align"}
// Dependencies: {}
fn size_align < T > () -> (usize , usize) { (mem :: size_of :: < T > () , mem :: align_of :: < T > ()) }
};
}
