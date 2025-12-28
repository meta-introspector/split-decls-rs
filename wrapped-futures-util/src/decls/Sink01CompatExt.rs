macro_rules! deps {
    () => {
        Compat01As03Sink!();
    };
}

macro_rules! Sink01CompatExt {
    () => {
        deps!();
        # [doc = " Extension trait for futures 0.1 [`Sink`](futures_01::sink::Sink)"] # [cfg (feature = "sink")] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub trait Sink01CompatExt : Sink01 { # [doc = " Converts a futures 0.1"] # [doc = " [`Sink<SinkItem = T, SinkError = E>`](futures_01::sink::Sink)"] # [doc = " into a futures 0.3"] # [doc = " [`Sink<T, Error = E>`](futures_sink::Sink)."] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return; } // https://github.com/rust-lang/futures-rs/issues/2514"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::{sink::SinkExt, stream::StreamExt};"] # [doc = " use futures_util::compat::{Stream01CompatExt, Sink01CompatExt};"] # [doc = ""] # [doc = " let (tx, rx) = futures_01::unsync::mpsc::channel(1);"] # [doc = " let (mut tx, mut rx) = (tx.sink_compat(), rx.compat());"] # [doc = ""] # [doc = " tx.send(1).await.unwrap();"] # [doc = " drop(tx);"] # [doc = " assert_eq!(rx.next().await, Some(Ok(1)));"] # [doc = " assert_eq!(rx.next().await, None);"] # [doc = " # });"] # [doc = " ```"] fn sink_compat (self) -> Compat01As03Sink < Self , Self :: SinkItem > where Self : Sized , { Compat01As03Sink :: new (self) } }
    };
}

Sink01CompatExt!()