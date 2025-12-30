// Generated macro for set_file_atime (function)
macro_rules! Depcrateset_file_atime {
() => {
// Module: crate
// Provides: {"set_file_atime"}
// Dependencies: {}
# [doc = " Set the last access time for a file on the filesystem."] # [doc = ""] # [doc = " This function will set the `atime` metadata field for a file on the local"] # [doc = " filesystem, returning any error encountered."] # [doc = ""] # [doc = " # Platform support"] # [doc = ""] # [doc = " Where supported this will attempt to issue just one syscall to update only"] # [doc = " the `atime`, but where not supported this may issue one syscall to learn the"] # [doc = " existing `mtime` so only the `atime` can be configured."] pub fn set_file_atime < P > (p : P , atime : FileTime) -> io :: Result < () > where P : AsRef < Path > , { imp :: set_file_atime (p . as_ref () , atime) }
};
}
