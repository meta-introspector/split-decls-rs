// Generated macro for Error (enum)
macro_rules! Depcrate_data_file_decodeError {
() => {
// Module: crate::data::file::decode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned by [`File::decode_header()`][crate::data::File::decode_header()],"] # [doc = " [`File::decode_entry()`][crate::data::File::decode_entry()] and ."] # [doc = " [`File::decompress_entry()`][crate::data::File::decompress_entry()]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to decompress pack entry")] ZlibInflate (# [from] gix_features :: zlib :: inflate :: Error) , # [error ("A delta chain could not be followed as the ref base with id {0} could not be found")] DeltaBaseUnresolved (gix_hash :: ObjectId) , # [error (transparent)] EntryType (# [from] crate :: data :: entry :: decode :: Error) , # [error ("Entry too large to fit in memory")] OutOfMemory , # [error (transparent)] Delta (# [from] crate :: data :: delta :: apply :: Error) , }
};
}
