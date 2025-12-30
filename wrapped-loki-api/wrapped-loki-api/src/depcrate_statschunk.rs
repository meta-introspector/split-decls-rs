// Generated macro for Chunk (struct)
macro_rules! Depcrate_statsChunk {
() => {
// Module: crate::stats
// Provides: {"Chunk"}
// Dependencies: {}
# [derive (Clone , PartialEq , :: prost :: Message)] pub struct Chunk { # [doc = " Total bytes processed but was already in memory. (found in the headchunk)"] # [prost (int64 , tag = "4")] pub head_chunk_bytes : i64 , # [doc = " Total lines processed but was already in memory. (found in the headchunk)"] # [prost (int64 , tag = "5")] pub head_chunk_lines : i64 , # [doc = " Total bytes decompressed and processed from chunks."] # [prost (int64 , tag = "6")] pub decompressed_bytes : i64 , # [doc = " Total lines decompressed and processed from chunks."] # [prost (int64 , tag = "7")] pub decompressed_lines : i64 , # [doc = " Total bytes of compressed chunks (blocks) processed."] # [prost (int64 , tag = "8")] pub compressed_bytes : i64 , # [doc = " Total duplicates found while processing."] # [prost (int64 , tag = "9")] pub total_duplicates : i64 , }
};
}
