// Generated macro for raw_waker (function)
macro_rules! Depcrate_taskraw_waker {
() => {
// Module: crate::task
// Provides: {"raw_waker"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [inline (always)] fn raw_waker < W : Wake + Send + Sync + 'static > (waker : Arc < W >) -> RawWaker { # [inline (always)] unsafe fn clone_waker < W : Wake + Send + Sync + 'static > (waker : * const ()) -> RawWaker { unsafe { Arc :: increment_strong_count (waker as * const W) } ; RawWaker :: new (waker , & RawWakerVTable :: new (clone_waker :: < W > , wake :: < W > , wake_by_ref :: < W > , drop_waker :: < W >) ,) } unsafe fn wake < W : Wake + Send + Sync + 'static > (waker : * const ()) { let waker = unsafe { Arc :: from_raw (waker as * const W) } ; < W as Wake > :: wake (waker) ; } unsafe fn wake_by_ref < W : Wake + Send + Sync + 'static > (waker : * const ()) { let waker = unsafe { ManuallyDrop :: new (Arc :: from_raw (waker as * const W)) } ; < W as Wake > :: wake_by_ref (& waker) ; } unsafe fn drop_waker < W : Wake + Send + Sync + 'static > (waker : * const ()) { unsafe { Arc :: decrement_strong_count (waker as * const W) } ; } RawWaker :: new (Arc :: into_raw (waker) as * const () , & RawWakerVTable :: new (clone_waker :: < W > , wake :: < W > , wake_by_ref :: < W > , drop_waker :: < W >) ,) }
};
}
