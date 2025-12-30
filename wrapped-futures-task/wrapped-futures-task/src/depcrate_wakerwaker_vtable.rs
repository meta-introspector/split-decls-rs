// Generated macro for waker_vtable (function)
macro_rules! Depcrate_wakerwaker_vtable {
() => {
// Module: crate::waker
// Provides: {"waker_vtable"}
// Dependencies: {}
pub (super) fn waker_vtable < W : ArcWake + 'static > () -> & 'static RawWakerVTable { & RawWakerVTable :: new (clone_arc_raw :: < W > , wake_arc_raw :: < W > , wake_by_ref_arc_raw :: < W > , drop_arc_raw :: < W > ,) }
};
}
