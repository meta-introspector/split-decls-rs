// Generated macro for local_raw_waker (function)
macro_rules! Depcrate_tasklocal_raw_waker {
() => {
// Module: crate::task
// Provides: {"local_raw_waker"}
// Dependencies: {}
# [inline (always)] fn local_raw_waker < W : LocalWake + 'static > (waker : Rc < W >) -> RawWaker { # [inline (always)] unsafe fn clone_waker < W : LocalWake + 'static > (waker : * const ()) -> RawWaker { unsafe { Rc :: increment_strong_count (waker as * const W) } ; RawWaker :: new (waker , & RawWakerVTable :: new (clone_waker :: < W > , wake :: < W > , wake_by_ref :: < W > , drop_waker :: < W >) ,) } unsafe fn wake < W : LocalWake + 'static > (waker : * const ()) { let waker = unsafe { Rc :: from_raw (waker as * const W) } ; < W as LocalWake > :: wake (waker) ; } unsafe fn wake_by_ref < W : LocalWake + 'static > (waker : * const ()) { let waker = unsafe { ManuallyDrop :: new (Rc :: from_raw (waker as * const W)) } ; < W as LocalWake > :: wake_by_ref (& waker) ; } unsafe fn drop_waker < W : LocalWake + 'static > (waker : * const ()) { unsafe { Rc :: decrement_strong_count (waker as * const W) } ; } RawWaker :: new (Rc :: into_raw (waker) as * const () , & RawWakerVTable :: new (clone_waker :: < W > , wake :: < W > , wake_by_ref :: < W > , drop_waker :: < W >) ,) }
};
}
