macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! Merge {
    () => {
        deps!();
        # [doc = " Combines multiple streams into a single stream of all their outputs."] # [doc = ""] # [doc = " Items are yielded as soon as they're received, and the stream continues"] # [doc = " yield until both streams have been exhausted. The output ordering"] # [doc = " between streams is not guaranteed."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_concurrency::prelude::*;"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = " use futures_lite::future::block_on;"] # [doc = ""] # [doc = " block_on(async {"] # [doc = "     let a = stream::once(1);"] # [doc = "     let b = stream::once(2);"] # [doc = "     let c = stream::once(3);"] # [doc = "     let mut s = [a, b, c].merge();"] # [doc = ""] # [doc = "     let mut buf = vec![];"] # [doc = "     s.for_each(|n| buf.push(n)).await;"] # [doc = "     buf.sort_unstable();"] # [doc = "     assert_eq!(&buf, &[1, 2, 3]);"] # [doc = " })"] # [doc = " ```"] pub trait Merge { # [doc = " The resulting output type."] type Item ; # [doc = " The stream type."] type Stream : Stream < Item = Self :: Item > ; # [doc = " Combine multiple streams into a single stream."] fn merge (self) -> Self :: Stream ; }
    };
}

Merge!();