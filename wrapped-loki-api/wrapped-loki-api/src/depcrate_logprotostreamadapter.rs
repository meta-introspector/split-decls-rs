// Generated macro for StreamAdapter (struct)
macro_rules! Depcrate_logprotoStreamAdapter {
() => {
// Module: crate::logproto
// Provides: {"StreamAdapter"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct StreamAdapter { # [prost (string , tag = "1")] pub labels : :: prost :: alloc :: string :: String , # [prost (message , repeated , tag = "2")] pub entries : :: prost :: alloc :: vec :: Vec < EntryAdapter > , # [doc = " hash contains the original hash of the stream."] # [prost (uint64 , tag = "3")] pub hash : u64 , }
};
}
