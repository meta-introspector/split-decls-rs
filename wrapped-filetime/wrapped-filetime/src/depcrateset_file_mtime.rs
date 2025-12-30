// Generated macro for set_file_mtime (function)
macro_rules! Depcrateset_file_mtime {
() => {
// Module: crate
// Provides: {"set_file_mtime"}
// Dependencies: {}
# [doc = " Set the last modification time for a file on the filesystem."] # [doc = ""] # [doc = " This function will set the `mtime` metadata field for a file on the local"] # [doc = " filesystem, returning any error encountered."] # [doc = ""] # [doc = " # Platform support"] # [doc = ""] # [doc = " Where supported this will attempt to issue just one syscall to update only"] # [doc = " the `mtime`, but where not supported this may issue one syscall to learn the"] # [doc = " existing `atime` so only the `mtime` can be configured."] pub fn set_file_mtime < P > (p : P , mtime : FileTime) -> io :: Result < () > where P : AsRef < Path > , { imp :: set_file_mtime (p . as_ref () , mtime) }
};
}
