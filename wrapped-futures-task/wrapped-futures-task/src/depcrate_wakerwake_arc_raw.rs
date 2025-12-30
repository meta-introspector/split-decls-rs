// Generated macro for wake_arc_raw (function)
macro_rules! Depcrate_wakerwake_arc_raw {
() => {
// Module: crate::waker
// Provides: {"wake_arc_raw"}
// Dependencies: {}
unsafe fn wake_arc_raw < T : ArcWake + 'static > (data : * const ()) { let arc : Arc < T > = unsafe { Arc :: from_raw (data . cast :: < T > ()) } ; ArcWake :: wake (arc) ; }
};
}
