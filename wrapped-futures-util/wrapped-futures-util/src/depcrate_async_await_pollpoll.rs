// Generated macro for poll (function)
macro_rules! Depcrate_async_await_pollpoll {
() => {
// Module: crate::async_await::poll
// Provides: {"poll"}
// Dependencies: {}
# [doc (hidden)] pub fn poll < F : Future + Unpin > (future : F) -> PollOnce < F > { PollOnce { future } }
};
}
