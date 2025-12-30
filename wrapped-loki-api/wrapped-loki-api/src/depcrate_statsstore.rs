// Generated macro for Store (struct)
macro_rules! Depcrate_statsStore {
() => {
// Module: crate::stats
// Provides: {"Store"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct Store { # [doc = " The total of chunk reference fetched from index."] # [prost (int64 , tag = "1")] pub total_chunks_ref : i64 , # [doc = " Total number of chunks fetched."] # [prost (int64 , tag = "2")] pub total_chunks_downloaded : i64 , # [doc = " Time spent fetching chunks in nanoseconds."] # [prost (int64 , tag = "3")] pub chunks_download_time : i64 , # [prost (message , optional , tag = "4")] pub chunk : :: core :: option :: Option < Chunk > , }
};
}
