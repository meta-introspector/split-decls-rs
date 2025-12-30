// Generated macro for Boxed (type)
macro_rules! Depcrate_futureBoxed {
() => {
// Module: crate::future
// Provides: {"Boxed"}
// Dependencies: {}
# [doc = " Type alias for `Pin<Box<dyn Future<Output = T> + Send + 'static>>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future::{self, FutureExt};"] # [doc = ""] # [doc = " // These two lines are equivalent:"] # [doc = " let f1: future::Boxed<i32> = async { 1 + 2 }.boxed();"] # [doc = " let f2: future::Boxed<i32> = Box::pin(async { 1 + 2 });"] # [doc = " ```"] # [cfg (feature = "alloc")] pub type Boxed < T > = Pin < Box < dyn Future < Output = T > + Send + 'static > > ;
};
}
