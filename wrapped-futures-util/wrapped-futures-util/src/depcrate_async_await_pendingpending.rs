// Generated macro for pending (macro)
macro_rules! Depcrate_async_await_pendingpending {
() => {
// Module: crate::async_await::pending
// Provides: {"pending"}
// Dependencies: {}
# [doc = " A macro which yields to the event loop once."] # [doc = ""] # [doc = " This is equivalent to returning [`Poll::Pending`](futures_core::task::Poll)"] # [doc = " from a [`Future::poll`](futures_core::future::Future::poll) implementation."] # [doc = " Similarly, when using this macro, it must be ensured that [`wake`](std::task::Waker::wake)"] # [doc = " is called somewhere when further progress can be made."] # [doc = ""] # [doc = " This macro is only usable inside of async functions, closures, and blocks."] # [doc = " It is also gated behind the `async-await` feature of this library, which is"] # [doc = " activated by default."] # [macro_export] macro_rules ! pending { () => { $ crate :: __private :: async_await :: pending_once () . await } ; }
};
}
