// Generated macro for impl_61 (impl)
macro_rules! Depcrate_task_wake_counterimpl_61 {
() => {
// Module: crate::task::wake_counter
// Provides: {"impl_61"}
// Dependencies: {}
impl ArcWake for WakerInner { fn wake_by_ref (arc_self : & Arc < Self >) { let _ = arc_self . count . fetch_add (1 , Ordering :: SeqCst) ; } }
};
}
