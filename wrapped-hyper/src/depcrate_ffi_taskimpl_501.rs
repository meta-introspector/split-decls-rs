// Generated macro for impl_501 (impl)
macro_rules! Depcrate_ffi_taskimpl_501 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_501"}
// Dependencies: {}
impl futures_util :: task :: ArcWake for ExecWaker { fn wake_by_ref (me : & Arc < ExecWaker >) { me . 0 . store (true , Ordering :: SeqCst) ; } }
};
}
