// Generated macro for once (function)
macro_rules! Depcrate_stream_onceonce {
() => {
// Module: crate::stream::once
// Provides: {"once"}
// Dependencies: {}
# [doc = " Creates a stream of a single element."] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::stream::{self, StreamExt};"] # [doc = ""] # [doc = " let stream = stream::once(async { 17 });"] # [doc = " let collected = stream.collect::<Vec<i32>>().await;"] # [doc = " assert_eq!(collected, vec![17]);"] # [doc = " # });"] # [doc = " ```"] pub fn once < Fut : Future > (future : Fut) -> Once < Fut > { assert_stream :: < Fut :: Output , _ > (Once :: new (future)) }
};
}
