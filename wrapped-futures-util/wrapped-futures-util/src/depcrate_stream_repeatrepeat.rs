// Generated macro for repeat (function)
macro_rules! Depcrate_stream_repeatrepeat {
() => {
// Module: crate::stream::repeat
// Provides: {"repeat"}
// Dependencies: {}
# [doc = " Create a stream which produces the same item repeatedly."] # [doc = ""] # [doc = " The stream never terminates. Note that you likely want to avoid"] # [doc = " usage of `collect` or such on the returned stream as it will exhaust"] # [doc = " available memory as it tries to just fill up all RAM."] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::stream::{self, StreamExt};"] # [doc = ""] # [doc = " let stream = stream::repeat(9);"] # [doc = " assert_eq!(vec![9, 9, 9], stream.take(3).collect::<Vec<i32>>().await);"] # [doc = " # });"] # [doc = " ```"] pub fn repeat < T > (item : T) -> Repeat < T > where T : Clone , { assert_stream :: < T , _ > (Repeat { item }) }
};
}
