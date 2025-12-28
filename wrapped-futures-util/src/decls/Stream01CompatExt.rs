macro_rules! deps {
    () => {
        Compat01As03!();
    };
}

macro_rules! Stream01CompatExt {
    () => {
        deps!();
        # [doc = " Extension trait for futures 0.1 [`Stream`](futures_01::stream::Stream)"] pub trait Stream01CompatExt : Stream01 { # [doc = " Converts a futures 0.1"] # [doc = " [`Stream<Item = T, Error = E>`](futures_01::stream::Stream)"] # [doc = " into a futures 0.3"] # [doc = " [`Stream<Item = Result<T, E>>`](futures_core::stream::Stream)."] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return; } // https://github.com/rust-lang/futures-rs/issues/2514"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::stream::StreamExt;"] # [doc = " use futures_util::compat::Stream01CompatExt;"] # [doc = ""] # [doc = " let stream = futures_01::stream::once::<u32, ()>(Ok(1));"] # [doc = " let mut stream = stream.compat();"] # [doc = " assert_eq!(stream.next().await, Some(Ok(1)));"] # [doc = " assert_eq!(stream.next().await, None);"] # [doc = " # });"] # [doc = " ```"] fn compat (self) -> Compat01As03 < Self > where Self : Sized , { Compat01As03 :: new (self) } }
    };
}

Stream01CompatExt!();