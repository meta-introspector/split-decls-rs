// Generated macro for new_count_waker (function)
macro_rules! Depcrate_task_wake_counternew_count_waker {
() => {
// Module: crate::task::wake_counter
// Provides: {"new_count_waker"}
// Dependencies: {}
# [doc = " Create a new [`Waker`] that counts the number of times it's awoken."] # [doc = ""] # [doc = " [`Waker`]: futures_core::task::Waker"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_test::task::new_count_waker;"] # [doc = ""] # [doc = " let (waker, count) = new_count_waker();"] # [doc = ""] # [doc = " assert_eq!(count, 0);"] # [doc = ""] # [doc = " waker.wake_by_ref();"] # [doc = " waker.wake();"] # [doc = ""] # [doc = " assert_eq!(count, 2);"] # [doc = " ```"] pub fn new_count_waker () -> (Waker , AwokenCount) { let inner = Arc :: new (WakerInner { count : AtomicUsize :: new (0) }) ; (task :: waker (inner . clone ()) , AwokenCount { inner }) }
};
}
