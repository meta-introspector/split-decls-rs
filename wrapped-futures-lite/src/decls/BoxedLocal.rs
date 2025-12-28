macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! BoxedLocal {
    () => {
        deps!();
        # [doc = " Type alias for `Pin<Box<dyn Stream<Item = T> + 'static>>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " // These two lines are equivalent:"] # [doc = " let s1: stream::BoxedLocal<i32> = stream::once(7).boxed_local();"] # [doc = " let s2: stream::BoxedLocal<i32> = Box::pin(stream::once(7));"] # [doc = " ```"] # [cfg (feature = "alloc")] pub type BoxedLocal < T > = Pin < Box < dyn Stream < Item = T > + 'static > > ;
    };
}

BoxedLocal!()