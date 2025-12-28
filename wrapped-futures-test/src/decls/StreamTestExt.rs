macro_rules! deps {
    () => {
        InterleavePending!();
        AssertUnmoved!();
    };
}

macro_rules! StreamTestExt {
    () => {
        deps!();
        # [doc = " Additional combinators for testing streams."] pub trait StreamTestExt : Stream { # [doc = " Asserts that the given is not moved after being polled."] # [doc = ""] # [doc = " A check for movement is performed each time the stream is polled"] # [doc = " and when `Drop` is called."] # [doc = ""] # [doc = " Aside from keeping track of the location at which the stream was first"] # [doc = " polled and providing assertions, this stream adds no runtime behavior"] # [doc = " and simply delegates to the child stream."] fn assert_unmoved (self) -> AssertUnmoved < Self > where Self : Sized , { AssertUnmoved :: new (self) } # [doc = " Introduces an extra [`Poll::Pending`](futures_core::task::Poll::Pending)"] # [doc = " in between each item of the stream."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::pin::pin;"] # [doc = ""] # [doc = " use futures::task::Poll;"] # [doc = " use futures::stream::{self, Stream};"] # [doc = " use futures_test::task::noop_context;"] # [doc = " use futures_test::stream::StreamTestExt;"] # [doc = ""] # [doc = " let stream = stream::iter(vec![1, 2]).interleave_pending();"] # [doc = " let mut stream = pin!(stream);"] # [doc = ""] # [doc = " let mut cx = noop_context();"] # [doc = ""] # [doc = " assert_eq!(stream.as_mut().poll_next(&mut cx), Poll::Pending);"] # [doc = " assert_eq!(stream.as_mut().poll_next(&mut cx), Poll::Ready(Some(1)));"] # [doc = " assert_eq!(stream.as_mut().poll_next(&mut cx), Poll::Pending);"] # [doc = " assert_eq!(stream.as_mut().poll_next(&mut cx), Poll::Ready(Some(2)));"] # [doc = " assert_eq!(stream.as_mut().poll_next(&mut cx), Poll::Pending);"] # [doc = " assert_eq!(stream.as_mut().poll_next(&mut cx), Poll::Ready(None));"] # [doc = " ```"] fn interleave_pending (self) -> InterleavePending < Self > where Self : Sized , { InterleavePending :: new (self) } }
    };
}

StreamTestExt!();