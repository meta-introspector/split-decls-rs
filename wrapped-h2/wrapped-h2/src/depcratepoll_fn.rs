// Generated macro for poll_fn (function)
macro_rules! Depcratepoll_fn {
() => {
// Module: crate
// Provides: {"poll_fn"}
// Dependencies: {}
# [doc = " Creates a future from a function that returns `Poll`."] fn poll_fn < T , F : FnMut (& mut Context < '_ >) -> T > (f : F) -> PollFn < F > { PollFn (f) }
};
}
