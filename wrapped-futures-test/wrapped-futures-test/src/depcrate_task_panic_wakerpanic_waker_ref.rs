// Generated macro for panic_waker_ref (function)
macro_rules! Depcrate_task_panic_wakerpanic_waker_ref {
() => {
// Module: crate::task::panic_waker
// Provides: {"panic_waker_ref"}
// Dependencies: {}
# [doc = " Get a global reference to a"] # [doc = " [`Waker`](futures_core::task::Waker) referencing a singleton"] # [doc = " instance of a [`Waker`] which panics when woken."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use futures_test::task::panic_waker_ref;"] # [doc = ""] # [doc = " let waker = panic_waker_ref();"] # [doc = " waker.wake_by_ref(); // Will panic"] # [doc = " ```"] pub fn panic_waker_ref () -> & 'static Waker { struct SyncRawWaker (RawWaker) ; unsafe impl Sync for SyncRawWaker { } static PANIC_WAKER_INSTANCE : SyncRawWaker = SyncRawWaker (raw_panic_waker ()) ; unsafe { & * (& PANIC_WAKER_INSTANCE . 0 as * const RawWaker as * const Waker) } }
};
}
