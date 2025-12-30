// Generated macro for increase_refcount (function)
macro_rules! Depcrate_wakerincrease_refcount {
() => {
// Module: crate::waker
// Provides: {"increase_refcount"}
// Dependencies: {}
unsafe fn increase_refcount < T : ArcWake + 'static > (data : * const ()) { let arc = mem :: ManuallyDrop :: new (unsafe { Arc :: < T > :: from_raw (data . cast :: < T > ()) }) ; let _arc_clone : mem :: ManuallyDrop < _ > = arc . clone () ; }
};
}
