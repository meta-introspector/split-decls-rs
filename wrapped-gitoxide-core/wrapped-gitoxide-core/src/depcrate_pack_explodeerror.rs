// Generated macro for Error (enum)
macro_rules! Depcrate_pack_explodeError {
() => {
// Module: crate::pack::explode
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , thiserror :: Error)] enum Error { # [error ("An IO error occurred while writing an object")] Io (# [from] std :: io :: Error) , # [error ("An object could not be written to the database")] OdbWrite (# [from] loose :: write :: Error) , # [error ("Failed to write {kind} object {id}")] Write { source : Box < dyn std :: error :: Error + Send + Sync > , kind : object :: Kind , id : ObjectId , } , # [error ("Object didn't verify after right after writing it")] Verify (# [from] objs :: data :: verify :: Error) , # [error ("{kind} object wasn't re-encoded without change")] ObjectEncodeMismatch { # [source] source : gix :: hash :: verify :: Error , kind : object :: Kind , } , # [error ("The recently written file for loose object {id} could not be found")] WrittenFileMissing { id : ObjectId } , # [error ("The recently written file for loose object {id} cold not be read")] WrittenFileCorrupt { source : loose :: find :: Error , id : ObjectId } , }
};
}
