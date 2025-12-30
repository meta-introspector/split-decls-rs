// Generated macro for poll_fn (function)
macro_rules! Depcrate_future_extpoll_fn {
() => {
// Module: crate::future_ext
// Provides: {"poll_fn"}
// Dependencies: {}
pub fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { PollFn { f } }
};
}
