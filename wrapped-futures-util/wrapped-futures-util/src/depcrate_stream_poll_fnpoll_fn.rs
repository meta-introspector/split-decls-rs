// Generated macro for poll_fn (function)
macro_rules! Depcrate_stream_poll_fnpoll_fn {
() => {
// Module: crate::stream::poll_fn
// Provides: {"poll_fn"}
// Dependencies: {}
# [doc = " Creates a new stream wrapping a function returning `Poll<Option<T>>`."] # [doc = ""] # [doc = " Polling the returned stream calls the wrapped function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::stream::poll_fn;"] # [doc = " use futures::task::Poll;"] # [doc = ""] # [doc = " let mut counter = 1usize;"] # [doc = ""] # [doc = " let read_stream = poll_fn(move |_| -> Poll<Option<String>> {"] # [doc = "     if counter == 0 { return Poll::Ready(None); }"] # [doc = "     counter -= 1;"] # [doc = "     Poll::Ready(Some(\"Hello, World!\".to_owned()))"] # [doc = " });"] # [doc = " ```"] pub fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < Option < T > > , { assert_stream :: < T , _ > (PollFn { f }) }
};
}
