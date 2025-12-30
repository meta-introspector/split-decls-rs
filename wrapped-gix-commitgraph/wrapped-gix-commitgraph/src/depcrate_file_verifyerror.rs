// Generated macro for Error (enum)
macro_rules! Depcrate_file_verifyError {
() => {
// Module: crate::file::verify
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in [`File::traverse()`]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error < E : std :: error :: Error + 'static > { # [error (transparent)] Commit (# [from] file :: commit :: Error) , # [error ("commit at file position {pos} has invalid ID {id}")] CommitId { id : gix_hash :: ObjectId , pos : file :: Position , } , # [error ("commit at file position {pos} with ID {id} is out of order relative to its predecessor with ID {predecessor_id}")] CommitsOutOfOrder { id : gix_hash :: ObjectId , pos : file :: Position , predecessor_id : gix_hash :: ObjectId , } , # [error ("commit-graph filename should be {0}")] Filename (String) , # [error ("commit {id} has invalid generation {generation}")] Generation { generation : u32 , id : gix_hash :: ObjectId } , # [error (transparent)] Checksum (# [from] checksum :: Error) , # [error ("{0}")] Processor (# [source] E) , # [error ("commit {id} has invalid root tree ID {root_tree_id}")] RootTreeId { id : gix_hash :: ObjectId , root_tree_id : gix_hash :: ObjectId , } , }
};
}
