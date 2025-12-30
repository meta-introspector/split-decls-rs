// Generated macro for Error (enum)
macro_rules! Depcrate_verifyError {
() => {
// Module: crate::verify
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in [`verify_integrity()`][Graph::verify_integrity]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error < E : std :: error :: Error + 'static > { # [error ("'{}' should have {expected} base graphs, but claims {actual} base graphs" , . path . display ())] BaseGraphCount { actual : u8 , expected : u8 , path : PathBuf } , # [error ("'{}' base graph at index {index} should have ID {expected} but is {actual}" , . path . display ())] BaseGraphId { actual : gix_hash :: ObjectId , expected : gix_hash :: ObjectId , index : u8 , path : PathBuf , } , # [error (transparent)] Commit (# [from] commit :: Error) , # [error ("{}: {err}" , . path . display ())] File { err : file :: verify :: Error < std :: convert :: Infallible > , path : PathBuf , } , # [error ("Commit {id}'s generation should be {expected} but is {actual}")] Generation { actual : u32 , expected : u32 , id : gix_hash :: ObjectId , } , # [error ("Commit {id} has parent position {parent_pos} that is out of range (should be in range 0-{max_valid_pos})")] ParentOutOfRange { id : gix_hash :: ObjectId , max_valid_pos : Position , parent_pos : Position , } , # [error ("{0}")] Processor (# [source] E) , # [error ("Commit-graph should be composed of at most 256 files but actually contains {0} files")] TooManyFiles (usize) , }
};
}
