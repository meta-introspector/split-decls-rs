// Generated macro for repeat (function)
macro_rules! Depcrate_streamrepeat {
() => {
// Module: crate::stream
// Provides: {"repeat"}
// Dependencies: {}
# [doc = " Creates an infinite stream that yields the same item repeatedly."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut s = stream::repeat(7);"] # [doc = ""] # [doc = " assert_eq!(s.next().await, Some(7));"] # [doc = " assert_eq!(s.next().await, Some(7));"] # [doc = " # })"] # [doc = " ```"] pub fn repeat < T : Clone > (item : T) -> Repeat < T > { Repeat { item } }
};
}
