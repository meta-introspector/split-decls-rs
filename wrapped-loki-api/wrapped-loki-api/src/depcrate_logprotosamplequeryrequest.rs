// Generated macro for SampleQueryRequest (struct)
macro_rules! Depcrate_logprotoSampleQueryRequest {
() => {
// Module: crate::logproto
// Provides: {"SampleQueryRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct SampleQueryRequest { # [prost (string , tag = "1")] pub selector : :: prost :: alloc :: string :: String , # [prost (message , optional , tag = "2")] pub start : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (message , optional , tag = "3")] pub end : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (string , repeated , tag = "4")] pub shards : :: prost :: alloc :: vec :: Vec < :: prost :: alloc :: string :: String > , # [prost (message , repeated , tag = "5")] pub deletes : :: prost :: alloc :: vec :: Vec < Delete > , }
};
}
