// Generated macro for Delete (struct)
macro_rules! Depcrate_logprotoDelete {
() => {
// Module: crate::logproto
// Provides: {"Delete"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct Delete { # [prost (string , tag = "1")] pub selector : :: prost :: alloc :: string :: String , # [prost (int64 , tag = "2")] pub start : i64 , # [prost (int64 , tag = "3")] pub end : i64 , }
};
}
