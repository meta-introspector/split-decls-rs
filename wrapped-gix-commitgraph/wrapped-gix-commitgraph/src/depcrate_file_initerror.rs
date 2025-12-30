// Generated macro for Error (enum)
macro_rules! Depcrate_file_initError {
() => {
// Module: crate::file::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in [`File::at()`]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Commit-graph {:?} chunk contains {from_chunk} base graphs, but commit-graph file header claims {from_header} base graphs" , BASE_GRAPHS_LIST_CHUNK_ID . as_bstr ())] BaseGraphMismatch { from_header : u8 , from_chunk : u32 } , # [error ("Commit-graph {:?} chunk contains {chunk1_commits} commits, but {:?} chunk contains {chunk2_commits} commits" , . chunk1_id . as_bstr () , . chunk2_id . as_bstr ())] CommitCountMismatch { chunk1_id : ChunkId , chunk1_commits : u32 , chunk2_id : ChunkId , chunk2_commits : u32 , } , # [error ("{0}")] Corrupt (String) , # [error ("Could not open commit-graph file at '{}'" , . path . display ())] Io { # [source] err : std :: io :: Error , path : std :: path :: PathBuf , } , # [error ("{0}")] Trailer (String) , # [error ("Commit-graph file uses unsupported hash version: {0}")] UnsupportedHashVersion (u8) , # [error ("Unsupported commit-graph file version: {0}")] UnsupportedVersion (u8) , # [error (transparent)] ChunkFileDecode (# [from] gix_chunk :: file :: decode :: Error) , # [error (transparent)] MissingChunk (# [from] gix_chunk :: file :: index :: offset_by_kind :: Error) , # [error ("Commit-graph chunk {:?} has invalid size: {msg}" , . id . as_bstr ())] InvalidChunkSize { id : ChunkId , msg : String } , }
};
}
