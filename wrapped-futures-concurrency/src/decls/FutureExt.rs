macro_rules! deps {
    () => {
        WaitUntil!();
    };
}

macro_rules! FutureExt {
    () => {
        deps!();
        # [doc = " An extension trait for the `Future` trait."] pub trait FutureExt : Future { # [doc = " Wait for both futures to complete."] fn join < S2 > (self , other : S2) -> Join2 < Self , S2 :: IntoFuture > where Self : Future + Sized , S2 : IntoFuture ; # [doc = " Wait for the first future to complete."] fn race < T , S2 > (self , other : S2) -> Race2 < T , Self , S2 :: IntoFuture > where Self : Future < Output = T > + Sized , S2 : IntoFuture < Output = T > ; # [doc = " Delay resolving the future until the given deadline."] # [doc = ""] # [doc = " The underlying future will not be polled until the deadline has expired. In addition"] # [doc = " to using a time source as a deadline, any future can be used as a"] # [doc = " deadline too. When used in combination with a multi-consumer channel,"] # [doc = " this method can be used to synchronize the start of multiple futures and streams."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(miri)]fn main() {}"] # [doc = " # #[cfg(not(miri))]"] # [doc = " # fn main() {"] # [doc = " use async_io::Timer;"] # [doc = " use futures_concurrency::prelude::*;"] # [doc = " use futures_lite::future::block_on;"] # [doc = " use std::time::{Duration, Instant};"] # [doc = ""] # [doc = " block_on(async {"] # [doc = "     let now = Instant::now();"] # [doc = "     let duration = Duration::from_millis(100);"] # [doc = ""] # [doc = "     async { \"meow\" }"] # [doc = "         .wait_until(Timer::after(duration))"] # [doc = "         .await;"] # [doc = ""] # [doc = "     assert!(now.elapsed() >= duration);"] # [doc = " });"] # [doc = " # }"] # [doc = " ```"] fn wait_until < D > (self , deadline : D) -> WaitUntil < Self , D :: IntoFuture > where Self : Sized , D : IntoFuture , { WaitUntil :: new (self , deadline . into_future ()) } }
    };
}

FutureExt!();