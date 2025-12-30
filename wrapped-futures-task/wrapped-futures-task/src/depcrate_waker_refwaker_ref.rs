// Generated macro for waker_ref (function)
macro_rules! Depcrate_waker_refwaker_ref {
() => {
// Module: crate::waker_ref
// Provides: {"waker_ref"}
// Dependencies: {}
# [doc = " Creates a reference to a [`Waker`] from a reference to `Arc<impl ArcWake>`."] # [doc = ""] # [doc = " The resulting [`Waker`] will call"] # [doc = " [`ArcWake.wake()`](ArcWake::wake) if awoken."] # [inline] pub fn waker_ref < W > (wake : & Arc < W >) -> WakerRef < '_ > where W : ArcWake + 'static , { let ptr = Arc :: as_ptr (wake) . cast :: < () > () ; let waker = ManuallyDrop :: new (unsafe { Waker :: from_raw (RawWaker :: new (ptr , waker_vtable :: < W > ())) }) ; WakerRef :: new_unowned (waker) }
};
}
