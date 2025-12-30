// Generated macro for race (function)
macro_rules! Depcrate_streamrace {
() => {
// Module: crate::stream
// Provides: {"race"}
// Dependencies: {}
# [doc = " Merges two streams, with no preference for either stream when both are ready."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, once, pending, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(stream::race(once(1), pending()).next().await, Some(1));"] # [doc = " assert_eq!(stream::race(pending(), once(2)).next().await, Some(2));"] # [doc = ""] # [doc = " // One of the two stream is randomly chosen as the winner."] # [doc = " let res = stream::race(once(1), once(2)).next().await;"] # [doc = " # })"] # [doc = " ```"] # [cfg (all (feature = "std" , feature = "race"))] pub fn race < T , S1 , S2 > (stream1 : S1 , stream2 : S2) -> Race < S1 , S2 > where S1 : Stream < Item = T > , S2 : Stream < Item = T > , { Race { stream1 , stream2 , rng : Rng :: new () , } }
};
}
