// Generated macro for FeedResult (enum)
macro_rules! Depcrate_accumulatorFeedResult {
() => {
// Module: crate::accumulator
// Provides: {"FeedResult"}
// Dependencies: {}
# [doc = " The result of feeding the accumulator."] # [cfg_attr (feature = "use-defmt" , derive (defmt :: Format))] pub enum FeedResult < 'a , T > { # [doc = " Consumed all data, still pending."] Consumed , # [doc = " Buffer was filled. Contains remaining section of input, if any."] OverFull (& 'a [u8]) , # [doc = " Reached end of chunk, but deserialization failed. Contains remaining section of input, if any."] DeserError (& 'a [u8]) , # [doc = " Deserialization complete. Contains deserialized data and remaining section of input, if any."] Success { # [doc = " Deserialize data."] data : T , # [doc = " Remaining data left in the buffer after deserializing."] remaining : & 'a [u8] , } , }
};
}
