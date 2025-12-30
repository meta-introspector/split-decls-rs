// Generated macro for drop_arc_raw (function)
macro_rules! Depcrate_wakerdrop_arc_raw {
() => {
// Module: crate::waker
// Provides: {"drop_arc_raw"}
// Dependencies: {}
unsafe fn drop_arc_raw < T : ArcWake + 'static > (data : * const ()) { drop (unsafe { Arc :: < T > :: from_raw (data . cast :: < T > ()) }) }
};
}
