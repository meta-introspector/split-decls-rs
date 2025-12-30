// Generated macro for optimistic (function)
macro_rules! Depcrateoptimistic {
() => {
// Module: crate
// Provides: {"optimistic"}
// Dependencies: {}
# [doc = " Polls a future once, waits for a wakeup, and then optimistically assumes the future is ready."] async fn optimistic (fut : impl Future < Output = io :: Result < () > >) -> io :: Result < () > { let mut polled = false ; let mut fut = pin ! (fut) ; poll_fn (| cx | { if ! polled { polled = true ; fut . as_mut () . poll (cx) } else { Poll :: Ready (Ok (())) } }) . await }
};
}
