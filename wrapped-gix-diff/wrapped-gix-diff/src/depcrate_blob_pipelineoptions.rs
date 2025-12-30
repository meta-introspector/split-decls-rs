// Generated macro for Options (struct)
macro_rules! Depcrate_blob_pipelineOptions {
() => {
// Module: crate::blob::pipeline
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in a [`Pipeline`]."] # [derive (Default , Clone , Copy , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct Options { # [doc = " The amount of bytes that an object has to reach before being treated as binary."] # [doc = " These objects will not be queried, nor will their data be processed in any way."] # [doc = " If `0`, no file is ever considered binary due to their size."] # [doc = ""] # [doc = " Note that for files stored in `git`, what counts is their stored, decompressed size,"] # [doc = " thus `git-lfs` files would typically not be considered binary unless one explicitly sets"] # [doc = " them"] pub large_file_threshold_bytes : u64 , # [doc = " Capabilities of the file system which affect how we read worktree files."] pub fs : gix_fs :: Capabilities , }
};
}
