// Generated macro for MergedItem (enum)
macro_rules! Depcrate_stream_mergeMergedItem {
() => {
// Module: crate::stream::merge
// Provides: {"MergedItem"}
// Dependencies: {}
# [doc = " An item returned from a merge stream, which represents an item from one or"] # [doc = " both of the underlying streams."] pub enum MergedItem < I1 , I2 > { # [doc = " An item from the first stream"] First (I1) , # [doc = " An item from the second stream"] Second (I2) , # [doc = " Items from both streams"] Both (I1 , I2) , }
};
}
