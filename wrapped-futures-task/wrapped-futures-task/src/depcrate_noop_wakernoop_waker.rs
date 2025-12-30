// Generated macro for noop_waker (function)
macro_rules! Depcrate_noop_wakernoop_waker {
() => {
// Module: crate::noop_waker
// Provides: {"noop_waker"}
// Dependencies: {}
# [doc = " Create a new [`Waker`] which does"] # [doc = " nothing when `wake()` is called on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::noop_waker;"] # [doc = " let waker = noop_waker();"] # [doc = " waker.wake();"] # [doc = " ```"] # [inline] pub fn noop_waker () -> Waker { unsafe { Waker :: from_raw (noop_raw_waker ()) } }
};
}
