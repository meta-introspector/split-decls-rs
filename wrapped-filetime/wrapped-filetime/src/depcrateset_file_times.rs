// Generated macro for set_file_times (function)
macro_rules! Depcrateset_file_times {
() => {
// Module: crate
// Provides: {"set_file_times"}
// Dependencies: {}
# [doc = " Set the last access and modification times for a file on the filesystem."] # [doc = ""] # [doc = " This function will set the `atime` and `mtime` metadata fields for a file"] # [doc = " on the local filesystem, returning any error encountered."] pub fn set_file_times < P > (p : P , atime : FileTime , mtime : FileTime) -> io :: Result < () > where P : AsRef < Path > , { imp :: set_file_times (p . as_ref () , atime , mtime) }
};
}
