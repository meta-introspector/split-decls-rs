// Generated macro for wake_by_ref_arc_raw (function)
macro_rules! Depcrate_wakerwake_by_ref_arc_raw {
() => {
// Module: crate::waker
// Provides: {"wake_by_ref_arc_raw"}
// Dependencies: {}
unsafe fn wake_by_ref_arc_raw < T : ArcWake + 'static > (data : * const ()) { let arc = mem :: ManuallyDrop :: new (unsafe { Arc :: < T > :: from_raw (data . cast :: < T > ()) }) ; ArcWake :: wake_by_ref (& arc) ; }
};
}
