// Generated macro for Ingester (struct)
macro_rules! Depcrate_statsIngester {
() => {
// Module: crate::stats
// Provides: {"Ingester"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct Ingester { # [doc = " Total ingester reached for this query."] # [prost (int32 , tag = "1")] pub total_reached : i32 , # [doc = " Total of chunks matched by the query from ingesters"] # [prost (int64 , tag = "2")] pub total_chunks_matched : i64 , # [doc = " Total of batches sent from ingesters."] # [prost (int64 , tag = "3")] pub total_batches : i64 , # [doc = " Total lines sent by ingesters."] # [prost (int64 , tag = "4")] pub total_lines_sent : i64 , # [prost (message , optional , tag = "5")] pub store : :: core :: option :: Option < Store > , }
};
}
