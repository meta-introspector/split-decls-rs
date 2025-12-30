// Generated macro for async_timeout (function)
macro_rules! Depcrate_interact_sessionasync_timeout {
() => {
// Module: crate::interact::session
// Provides: {"async_timeout"}
// Dependencies: {}
# [cfg (feature = "async")] async fn async_timeout (timeout : Duration) -> io :: Result < usize > { Delay :: new (timeout) . await ; io :: Result :: Ok (0) }
};
}
