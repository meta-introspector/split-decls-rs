// Generated macro for noop_waker_ref (function)
macro_rules! Depcrate_noop_wakernoop_waker_ref {
() => {
// Module: crate::noop_waker
// Provides: {"noop_waker_ref"}
// Dependencies: {}
# [doc = " Get a static reference to a [`Waker`] which"] # [doc = " does nothing when `wake()` is called on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::noop_waker_ref;"] # [doc = " let waker = noop_waker_ref();"] # [doc = " waker.wake_by_ref();"] # [doc = " ```"] # [inline] pub fn noop_waker_ref () -> & 'static Waker { struct SyncRawWaker (RawWaker) ; unsafe impl Sync for SyncRawWaker { } static NOOP_WAKER_INSTANCE : SyncRawWaker = SyncRawWaker (noop_raw_waker ()) ; unsafe { & * (& NOOP_WAKER_INSTANCE . 0 as * const RawWaker as * const Waker) } }
};
}
