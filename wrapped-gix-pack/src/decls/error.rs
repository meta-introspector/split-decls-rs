macro_rules! deps {
    () => {
        Error!();
        Id!();
    };
}

macro_rules! error {
    () => {
        deps!();
        mod error { use crate :: multi_index :: chunk ; # [doc = " The error returned by [File::at()][super::File::at()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not open multi-index file at '{path}'")] Io { source : std :: io :: Error , path : std :: path :: PathBuf , } , # [error ("{message}")] Corrupt { message : & 'static str } , # [error ("Unsupported multi-index version: {version})")] UnsupportedVersion { version : u8 } , # [error ("Unsupported hash kind: {kind})")] UnsupportedObjectHash { kind : u8 } , # [error (transparent)] ChunkFileDecode (# [from] gix_chunk :: file :: decode :: Error) , # [error (transparent)] MissingChunk (# [from] gix_chunk :: file :: index :: offset_by_kind :: Error) , # [error (transparent)] FileTooLarge (# [from] gix_chunk :: file :: index :: data_by_kind :: Error) , # [error ("The multi-pack fan doesn't have the correct size of 256 * 4 bytes")] MultiPackFanSize , # [error (transparent)] PackNames (# [from] chunk :: index_names :: decode :: Error) , # [error ("multi-index chunk {:?} has invalid size: {message}" , String :: from_utf8_lossy (. id))] InvalidChunkSize { id : gix_chunk :: Id , message : & 'static str } , } }
    };
}

error!()