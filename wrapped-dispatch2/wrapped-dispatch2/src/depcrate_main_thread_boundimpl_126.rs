// Generated macro for impl_126 (impl)
macro_rules! Depcrate_main_thread_boundimpl_126 {
() => {
// Module: crate::main_thread_bound
// Provides: {"impl_126"}
// Dependencies: {}
impl < T > Drop for MainThreadBound < T > { fn drop (& mut self) { if mem :: needs_drop :: < T > () { run_on_main (| _mtm | { let this = self ; unsafe { ManuallyDrop :: drop (& mut this . 0) } ; }) } } }
};
}
