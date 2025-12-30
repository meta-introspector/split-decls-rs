// Generated macro for block_on (function)
macro_rules! Depcrate_streamblock_on {
() => {
// Module: crate::stream
// Provides: {"block_on"}
// Dependencies: {}
# [doc = " Converts a stream into a blocking iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::{pin, stream};"] # [doc = ""] # [doc = " let stream = stream::once(7);"] # [doc = " pin!(stream);"] # [doc = ""] # [doc = " let mut iter = stream::block_on(stream);"] # [doc = " assert_eq!(iter.next(), Some(7));"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " ```"] # [cfg (feature = "std")] pub fn block_on < S : Stream + Unpin > (stream : S) -> BlockOn < S > { BlockOn (stream) }
};
}
