// Generated macro for poll_fn (function)
macro_rules! Depcrate_common_futurepoll_fn {
() => {
// Module: crate::common::future
// Provides: {"poll_fn"}
// Dependencies: {}
pub (crate) fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { PollFn { f } }
};
}
