macro_rules! deps {
    () => {
        WaitUntil!();
        IntoStream!();
        FromStream!();
    };
}

macro_rules! StreamExt {
    () => {
        deps!();
        # [doc = " An extension trait for the `Stream` trait."] pub trait StreamExt : Stream { # [doc = " Combines two streams into a single stream of all their outputs."] fn merge < T , S2 > (self , other : S2) -> Merge2 < T , Self , S2 :: IntoStream > where Self : Stream < Item = T > + Sized , S2 : IntoStream < Item = T > ; # [doc = " Takes two streams and creates a new stream over all in sequence"] fn chain < T , S2 > (self , other : S2) -> Chain2 < Self , S2 :: IntoStream > where Self : Stream < Item = T > + Sized , S2 : IntoStream < Item = T > ; # [doc = " ‘Zips up’ multiple streams into a single stream of pairs."] fn zip < T , S2 > (self , other : S2) -> Zip2 < Self , S2 :: IntoStream > where Self : Stream < Item = T > + Sized , S2 : IntoStream < Item = T > ; # [doc = " Convert into a concurrent stream."] # [cfg (feature = "alloc")] fn co (self) -> FromStream < Self > where Self : Sized , { FromStream :: new (self) } # [doc = " Delay the yielding of items from the stream until the given deadline."] # [doc = ""] # [doc = " The underlying stream will not be polled until the deadline has expired. In addition"] # [doc = " to using a time source as a deadline, any future can be used as a"] # [doc = " deadline too. When used in combination with a multi-consumer channel,"] # [doc = " this method can be used to synchronize the start of multiple streams and futures."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #[cfg(miri)] fn main() {}"] # [doc = " # #[cfg(not(miri))]"] # [doc = " # fn main() {"] # [doc = " use async_io::Timer;"] # [doc = " use futures_concurrency::prelude::*;"] # [doc = " use futures_lite::{future::block_on, stream};"] # [doc = " use futures_lite::prelude::*;"] # [doc = " use std::time::{Duration, Instant};"] # [doc = ""] # [doc = " block_on(async {"] # [doc = "     let now = Instant::now();"] # [doc = "     let duration = Duration::from_millis(100);"] # [doc = ""] # [doc = "     stream::once(\"meow\")"] # [doc = "         .wait_until(Timer::after(duration))"] # [doc = "         .next()"] # [doc = "         .await;"] # [doc = ""] # [doc = "     assert!(now.elapsed() >= duration);"] # [doc = " });"] # [doc = " # }"] # [doc = " ```"] fn wait_until < D > (self , deadline : D) -> WaitUntil < Self , D :: IntoFuture > where Self : Sized , D : IntoFuture , { WaitUntil :: new (self , deadline . into_future ()) } }
    };
}

StreamExt!()