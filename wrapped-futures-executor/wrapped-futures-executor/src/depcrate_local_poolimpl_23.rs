// Generated macro for impl_23 (impl)
macro_rules! Depcrate_local_poolimpl_23 {
() => {
// Module: crate::local_pool
// Provides: {"impl_23"}
// Dependencies: {}
impl ArcWake for ThreadNotify { fn wake_by_ref (arc_self : & Arc < Self >) { let unparked = arc_self . unparked . swap (true , Ordering :: Release) ; if ! unparked { arc_self . thread . unpark () ; } } }
};
}
