// Generated macro for impl_74 (impl)
macro_rules! Depcrate_thread_poolimpl_74 {
() => {
// Module: crate::thread_pool
// Provides: {"impl_74"}
// Dependencies: {}
impl ArcWake for WakeHandle { fn wake_by_ref (arc_self : & Arc < Self >) { if let Ok (task) = arc_self . mutex . notify () { arc_self . exec . state . send (Message :: Run (task)) } } }
};
}
