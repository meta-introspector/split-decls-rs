// Generated macro for FileMetadata (struct)
macro_rules! Depcrate_shims_unix_fsFileMetadata {
() => {
// Module: crate::shims::unix::fs
// Provides: {"FileMetadata"}
// Dependencies: {}
# [doc = " Stores a file's metadata in order to avoid code duplication in the different metadata related"] # [doc = " shims."] struct FileMetadata { mode : Scalar , size : u64 , created : Option < (u64 , u32) > , accessed : Option < (u64 , u32) > , modified : Option < (u64 , u32) > , dev : u64 , uid : u32 , gid : u32 , }
};
}
