macro_rules! deps {
    () => {
        Keyed!();
        Key!();
        StreamExt!();
        StreamGroup!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < S : Stream > StreamGroup < S > { # [doc = " Insert a new future into the group."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use futures_concurrency::stream::StreamGroup;"] # [doc = " use futures_lite::stream;"] # [doc = ""] # [doc = " let mut group = StreamGroup::with_capacity(2);"] # [doc = " group.insert(stream::once(12));"] # [doc = " ```"] pub fn insert (& mut self , stream : S) -> Key where S : Stream , { if self . capacity <= self . len () { self . reserve (self . capacity * 2 + 1) ; } let index = self . streams . insert (stream) ; self . keys . insert (index) ; self . states [index] . set_pending () ; self . wakers . readiness () . set_ready (index) ; Key (index) } # [doc = " Create a stream which also yields the key of each item."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use futures_concurrency::stream::StreamGroup;"] # [doc = " use futures_lite::{stream, StreamExt};"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let mut group = StreamGroup::new();"] # [doc = " group.insert(stream::once(2));"] # [doc = " group.insert(stream::once(4));"] # [doc = ""] # [doc = " let mut out = 0;"] # [doc = " let mut group = group.keyed();"] # [doc = " while let Some((_key, num)) = group.next().await {"] # [doc = "     out += num;"] # [doc = " }"] # [doc = " assert_eq!(out, 6);"] # [doc = " # });"] # [doc = " ```"] pub fn keyed (self) -> Keyed < S > { Keyed { group : self } } }
    };
}

impl_391!();