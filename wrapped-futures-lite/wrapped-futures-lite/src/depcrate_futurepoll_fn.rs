// Generated macro for poll_fn (function)
macro_rules! Depcrate_futurepoll_fn {
() => {
// Module: crate::future
// Provides: {"poll_fn"}
// Dependencies: {}
# [doc = " Creates a future from a function returning [`Poll`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future;"] # [doc = " use std::task::{Context, Poll};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " fn f(_: &mut Context<'_>) -> Poll<i32> {"] # [doc = "     Poll::Ready(7)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(future::poll_fn(f).await, 7);"] # [doc = " # })"] # [doc = " ```"] pub fn poll_fn < T , F > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { PollFn { f } }
};
}
