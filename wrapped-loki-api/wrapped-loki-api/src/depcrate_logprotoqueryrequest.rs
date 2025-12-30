// Generated macro for QueryRequest (struct)
macro_rules! Depcrate_logprotoQueryRequest {
() => {
// Module: crate::logproto
// Provides: {"QueryRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct QueryRequest { # [prost (string , tag = "1")] pub selector : :: prost :: alloc :: string :: String , # [prost (uint32 , tag = "2")] pub limit : u32 , # [prost (message , optional , tag = "3")] pub start : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (message , optional , tag = "4")] pub end : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (enumeration = "Direction" , tag = "5")] pub direction : i32 , # [prost (string , repeated , tag = "7")] pub shards : :: prost :: alloc :: vec :: Vec < :: prost :: alloc :: string :: String > , # [prost (message , repeated , tag = "8")] pub deletes : :: prost :: alloc :: vec :: Vec < Delete > , }
};
}
