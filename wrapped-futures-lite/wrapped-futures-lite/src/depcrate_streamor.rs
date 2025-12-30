// Generated macro for or (function)
macro_rules! Depcrate_streamor {
() => {
// Module: crate::stream
// Provides: {"or"}
// Dependencies: {}
# [doc = " Merges two streams, preferring items from `stream1` whenever both streams are ready."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, once, pending, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(stream::or(once(1), pending()).next().await, Some(1));"] # [doc = " assert_eq!(stream::or(pending(), once(2)).next().await, Some(2));"] # [doc = ""] # [doc = " // The first stream wins."] # [doc = " assert_eq!(stream::or(once(1), once(2)).next().await, Some(1));"] # [doc = " # })"] # [doc = " ```"] pub fn or < T , S1 , S2 > (stream1 : S1 , stream2 : S2) -> Or < S1 , S2 > where S1 : Stream < Item = T > , S2 : Stream < Item = T > , { Or { stream1 , stream2 } }
};
}
