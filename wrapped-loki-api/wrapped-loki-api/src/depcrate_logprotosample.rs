// Generated macro for Sample (struct)
macro_rules! Depcrate_logprotoSample {
() => {
// Module: crate::logproto
// Provides: {"Sample"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct Sample { # [prost (int64 , tag = "1")] pub timestamp : i64 , # [prost (double , tag = "2")] pub value : f64 , # [prost (uint64 , tag = "3")] pub hash : u64 , }
};
}
