// Generated macro for poll_fn (function)
macro_rules! Depcrate_future_poll_fnpoll_fn {
() => {
// Module: crate::future::poll_fn
// Provides: {"poll_fn"}
// Dependencies: {}
# [doc = " Creates a future driven by the provided function that receives a task context."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::task::Poll;"] # [doc = " # use actix_utils::future::poll_fn;"] # [doc = " # async fn test_poll_fn() {"] # [doc = " let res = poll_fn(|_| Poll::Ready(42)).await;"] # [doc = " assert_eq!(res, 42);"] # [doc = ""] # [doc = " let mut i = 5;"] # [doc = " let res = poll_fn(|cx| {"] # [doc = "     i -= 1;"] # [doc = ""] # [doc = "     if i > 0 {"] # [doc = "         cx.waker().wake_by_ref();"] # [doc = "         Poll::Pending"] # [doc = "     } else {"] # [doc = "         Poll::Ready(42)"] # [doc = "     }"] # [doc = " })"] # [doc = " .await;"] # [doc = " assert_eq!(res, 42);"] # [doc = " # }"] # [doc = " # actix_rt::Runtime::new().unwrap().block_on(test_poll_fn());"] # [doc = " ```"] # [inline] pub fn poll_fn < F , T > (f : F) -> PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { PollFn { f } }
};
}
