// Generated macro for TimeSeriesChunk (struct)
macro_rules! Depcrate_logprotoTimeSeriesChunk {
() => {
// Module: crate::logproto
// Provides: {"TimeSeriesChunk"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct TimeSeriesChunk { # [prost (string , tag = "1")] pub from_ingester_id : :: prost :: alloc :: string :: String , # [prost (string , tag = "2")] pub user_id : :: prost :: alloc :: string :: String , # [prost (message , repeated , tag = "3")] pub labels : :: prost :: alloc :: vec :: Vec < LabelPair > , # [prost (message , repeated , tag = "4")] pub chunks : :: prost :: alloc :: vec :: Vec < Chunk > , }
};
}
