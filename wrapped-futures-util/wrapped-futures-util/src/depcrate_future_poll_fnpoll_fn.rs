// Generated macro for poll_fn (function)
macro_rules! Depcrate_future_poll_fnpoll_fn {
() => {
// Module: crate::future::poll_fn
// Provides: {"poll_fn"}
// Dependencies: {}
# [doc = " Creates a new future wrapping around a function returning [`Poll`]."] # [doc = ""] # [doc = " Polling the returned future delegates to the wrapped function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future::poll_fn;"] # [doc = " use futures::task::{Context, Poll};"] # [doc = ""] # [doc = " fn read_line(_cx: &mut Context<'_>) -> Poll<String> {"] # [doc = "     Poll::Ready(\"Hello, World!\".into())"] # [doc = " }"] # [doc = ""] # [doc = " let read_future = poll_fn(read_line);"] # [doc = " assert_eq!(read_future.await, \"Hello, World!\".to_owned());"] # [doc = " # });"] # [doc = " ```"] pub fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { assert_future :: < T , _ > (PollFn { f }) }
};
}
