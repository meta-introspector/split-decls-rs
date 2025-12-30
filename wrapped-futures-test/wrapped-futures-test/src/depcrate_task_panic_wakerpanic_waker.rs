// Generated macro for panic_waker (function)
macro_rules! Depcrate_task_panic_wakerpanic_waker {
() => {
// Module: crate::task::panic_waker
// Provides: {"panic_waker"}
// Dependencies: {}
# [doc = " Create a new [`Waker`](futures_core::task::Waker) which will"] # [doc = " panic when `wake()` is called on it. The [`Waker`] can be converted"] # [doc = " into a [`Waker`] which will behave the same way."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use futures_test::task::panic_waker;"] # [doc = ""] # [doc = " let waker = panic_waker();"] # [doc = " waker.wake(); // Will panic"] # [doc = " ```"] pub fn panic_waker () -> Waker { unsafe { Waker :: from_raw (raw_panic_waker ()) } }
};
}
