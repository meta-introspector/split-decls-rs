// Generated macro for select (function)
macro_rules! Depcrate_stream_selectselect {
() => {
// Module: crate::stream::select
// Provides: {"select"}
// Dependencies: {}
# [doc = " This function will attempt to pull items from both streams. Each"] # [doc = " stream will be polled in a round-robin fashion, and whenever a stream is"] # [doc = " ready to yield an item that item is yielded."] # [doc = ""] # [doc = " After one of the two input streams completes, the remaining one will be"] # [doc = " polled exclusively. The returned stream completes when both input"] # [doc = " streams have completed."] # [doc = ""] # [doc = " Note that this function consumes both streams and returns a wrapped"] # [doc = " version of them."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::stream::{ repeat, select, StreamExt };"] # [doc = ""] # [doc = " let left = repeat(1);"] # [doc = " let right = repeat(2);"] # [doc = ""] # [doc = " let mut out = select(left, right);"] # [doc = ""] # [doc = " for _ in 0..100 {"] # [doc = "     // We should be alternating."] # [doc = "     assert_eq!(1, out.select_next_some().await);"] # [doc = "     assert_eq!(2, out.select_next_some().await);"] # [doc = " }"] # [doc = " # });"] # [doc = " ```"] pub fn select < St1 , St2 > (stream1 : St1 , stream2 : St2) -> Select < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { fn round_robin (last : & mut PollNext) -> PollNext { last . toggle () } assert_stream :: < St1 :: Item , _ > (Select { inner : select_with_strategy (stream1 , stream2 , round_robin) , }) }
};
}
