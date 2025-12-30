// Generated macro for Series (struct)
macro_rules! Depcrate_logprotoSeries {
() => {
// Module: crate::logproto
// Provides: {"Series"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct Series { # [prost (string , tag = "1")] pub labels : :: prost :: alloc :: string :: String , # [prost (message , repeated , tag = "2")] pub samples : :: prost :: alloc :: vec :: Vec < Sample > , # [prost (uint64 , tag = "3")] pub stream_hash : u64 , }
};
}
