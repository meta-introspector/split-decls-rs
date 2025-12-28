macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! Boxed {
    () => {
        deps!();
        # [doc = " Type alias for `Pin<Box<dyn Stream<Item = T> + Send + 'static>>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " // These two lines are equivalent:"] # [doc = " let s1: stream::Boxed<i32> = stream::once(7).boxed();"] # [doc = " let s2: stream::Boxed<i32> = Box::pin(stream::once(7));"] # [doc = " ```"] # [cfg (feature = "alloc")] pub type Boxed < T > = Pin < Box < dyn Stream < Item = T > + Send + 'static > > ;
    };
}

Boxed!();