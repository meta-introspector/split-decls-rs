// Generated macro for empty (function)
macro_rules! Depcrate_streamempty {
() => {
// Module: crate::stream
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Creates an empty stream."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut s = stream::empty::<i32>();"] # [doc = " assert_eq!(s.next().await, None);"] # [doc = " # })"] # [doc = " ```"] pub fn empty < T > () -> Empty < T > { Empty { _marker : PhantomData , } }
};
}
