// Generated macro for SeriesRequest (struct)
macro_rules! Depcrate_logprotoSeriesRequest {
() => {
// Module: crate::logproto
// Provides: {"SeriesRequest"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct SeriesRequest { # [prost (message , optional , tag = "1")] pub start : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (message , optional , tag = "2")] pub end : :: core :: option :: Option < :: prost_types :: Timestamp > , # [prost (string , repeated , tag = "3")] pub groups : :: prost :: alloc :: vec :: Vec < :: prost :: alloc :: string :: String > , # [prost (string , repeated , tag = "4")] pub shards : :: prost :: alloc :: vec :: Vec < :: prost :: alloc :: string :: String > , }
};
}
