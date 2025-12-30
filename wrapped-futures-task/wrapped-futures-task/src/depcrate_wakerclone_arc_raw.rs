// Generated macro for clone_arc_raw (function)
macro_rules! Depcrate_wakerclone_arc_raw {
() => {
// Module: crate::waker
// Provides: {"clone_arc_raw"}
// Dependencies: {}
# [inline (always)] unsafe fn clone_arc_raw < T : ArcWake + 'static > (data : * const ()) -> RawWaker { unsafe { increase_refcount :: < T > (data) } RawWaker :: new (data , waker_vtable :: < T > ()) }
};
}
