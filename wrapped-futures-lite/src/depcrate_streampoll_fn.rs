// Generated macro for poll_fn (function)
macro_rules! Depcrate_streampoll_fn {
() => {
// Module: crate::stream
// Provides: {"poll_fn"}
// Dependencies: {}
# [doc = " Creates a stream from a function returning [`Poll`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = " use std::task::{Context, Poll};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " fn f(_: &mut Context<'_>) -> Poll<Option<i32>> {"] # [doc = "     Poll::Ready(Some(7))"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(stream::poll_fn(f).next().await, Some(7));"] # [doc = " # })"] # [doc = " ```"] pub fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < Option < T > > , { PollFn { f } }
};
}
