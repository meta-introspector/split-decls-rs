macro_rules! noop_context {
    () => {
        # [doc = " Create a new [`Context`](core::task::Context) where the"] # [doc = " [waker](core::task::Context::waker) will ignore any uses."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::pin::pin;"] # [doc = ""] # [doc = " use futures::future::Future;"] # [doc = " use futures::task::Poll;"] # [doc = " use futures_test::task::noop_context;"] # [doc = ""] # [doc = " let future = async { 5 };"] # [doc = " let future = pin!(future);"] # [doc = ""] # [doc = " assert_eq!(future.poll(&mut noop_context()), Poll::Ready(5));"] # [doc = " ```"] pub fn noop_context () -> Context < 'static > { Context :: from_waker (noop_waker_ref ()) }
    };
}

noop_context!()