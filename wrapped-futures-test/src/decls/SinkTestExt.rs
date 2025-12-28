macro_rules! deps {
    () => {
        InterleavePending!();
        TrackClosed!();
        AssertUnmoved!();
    };
}

macro_rules! SinkTestExt {
    () => {
        deps!();
        # [doc = " Additional combinators for testing sinks."] pub trait SinkTestExt < Item > : Sink < Item > { # [doc = " Asserts that the given is not moved after being polled."] # [doc = ""] # [doc = " A check for movement is performed each time the sink is polled"] # [doc = " and when `Drop` is called."] # [doc = ""] # [doc = " Aside from keeping track of the location at which the sink was first"] # [doc = " polled and providing assertions, this sink adds no runtime behavior"] # [doc = " and simply delegates to the child sink."] fn assert_unmoved_sink (self) -> AssertUnmoved < Self > where Self : Sized , { AssertUnmoved :: new (self) } # [doc = " Introduces an extra [`Poll::Pending`](futures_core::task::Poll::Pending)"] # [doc = " in between each operation on the sink."] fn interleave_pending_sink (self) -> InterleavePending < Self > where Self : Sized , { InterleavePending :: new (self) } # [doc = " Track whether this sink has been closed and panics if it is used after closing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::sink::{SinkExt, drain};"] # [doc = " use futures_test::sink::SinkTestExt;"] # [doc = ""] # [doc = " let mut sink = drain::<i32>().track_closed();"] # [doc = ""] # [doc = " sink.send(1).await?;"] # [doc = " assert!(!sink.is_closed());"] # [doc = " sink.close().await?;"] # [doc = " assert!(sink.is_closed());"] # [doc = ""] # [doc = " # Ok::<(), std::convert::Infallible>(()) })?;"] # [doc = " # Ok::<(), std::convert::Infallible>(())"] # [doc = " ```"] # [doc = ""] # [doc = " Note: Unlike [`AsyncWriteTestExt::track_closed`] when"] # [doc = " used as a sink the adaptor will panic if closed too early as there's no easy way to"] # [doc = " integrate as an error."] # [doc = ""] # [doc = " [`AsyncWriteTestExt::track_closed`]: crate::io::AsyncWriteTestExt::track_closed"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use std::panic::AssertUnwindSafe;"] # [doc = " use futures::{sink::{SinkExt, drain}, future::FutureExt};"] # [doc = " use futures_test::sink::SinkTestExt;"] # [doc = ""] # [doc = " let mut sink = drain::<i32>().track_closed();"] # [doc = ""] # [doc = " sink.close().await?;"] # [doc = " assert!(AssertUnwindSafe(sink.send(1)).catch_unwind().await.is_err());"] # [doc = " # Ok::<(), std::convert::Infallible>(()) })?;"] # [doc = " # Ok::<(), std::convert::Infallible>(())"] # [doc = " ```"] fn track_closed (self) -> TrackClosed < Self > where Self : Sized , { TrackClosed :: new (self) } }
    };
}

SinkTestExt!();