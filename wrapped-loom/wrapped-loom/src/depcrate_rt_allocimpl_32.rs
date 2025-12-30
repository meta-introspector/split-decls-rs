// Generated macro for impl_32 (impl)
macro_rules! Depcrate_rt_allocimpl_32 {
() => {
// Module: crate::rt::alloc
// Provides: {"impl_32"}
// Dependencies: {}
impl Drop for Allocation { # [track_caller] fn drop (& mut self) { let location = location ! () ; rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; trace ! (state = ? self . state , drop . location = % location , "Allocation::drop") ; state . is_dropped = true ; }) ; } }
};
}
