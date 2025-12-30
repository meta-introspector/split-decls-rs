// Generated macro for panic_context (function)
macro_rules! Depcrate_task_contextpanic_context {
() => {
// Module: crate::task::context
// Provides: {"panic_context"}
// Dependencies: {}
# [doc = " Create a new [`Context`](core::task::Context) where the"] # [doc = " [waker](core::task::Context::waker) will panic if used."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use futures_test::task::panic_context;"] # [doc = ""] # [doc = " let cx = panic_context();"] # [doc = " cx.waker().wake_by_ref(); // Will panic"] # [doc = " ```"] pub fn panic_context () -> Context < 'static > { Context :: from_waker (panic_waker_ref ()) }
};
}
