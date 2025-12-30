// Generated macro for waker (function)
macro_rules! Depcrate_wakerwaker {
() => {
// Module: crate::waker
// Provides: {"waker"}
// Dependencies: {}
# [doc = " Creates a [`Waker`] from an `Arc<impl ArcWake>`."] # [doc = ""] # [doc = " The returned [`Waker`] will call"] # [doc = " [`ArcWake.wake()`](ArcWake::wake) if awoken."] pub fn waker < W > (wake : Arc < W >) -> Waker where W : ArcWake + 'static , { let ptr = Arc :: into_raw (wake) . cast :: < () > () ; unsafe { Waker :: from_raw (RawWaker :: new (ptr , waker_vtable :: < W > ())) } }
};
}
