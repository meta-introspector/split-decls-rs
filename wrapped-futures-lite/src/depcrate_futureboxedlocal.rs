// Generated macro for BoxedLocal (type)
macro_rules! Depcrate_futureBoxedLocal {
() => {
// Module: crate::future
// Provides: {"BoxedLocal"}
// Dependencies: {}
# [doc = " Type alias for `Pin<Box<dyn Future<Output = T> + 'static>>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future::{self, FutureExt};"] # [doc = ""] # [doc = " // These two lines are equivalent:"] # [doc = " let f1: future::BoxedLocal<i32> = async { 1 + 2 }.boxed_local();"] # [doc = " let f2: future::BoxedLocal<i32> = Box::pin(async { 1 + 2 });"] # [doc = " ```"] # [cfg (feature = "alloc")] pub type BoxedLocal < T > = Pin < Box < dyn Future < Output = T > + 'static > > ;
};
}
