// Generated macro for Error (enum)
macro_rules! Depcrate_fs_mountsError {
() => {
// Module: crate::fs::mounts
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum Error { # [cfg (target_os = "macos")] GetFSStatError (i32) , # [cfg (target_os = "linux")] IOError (std :: io :: Error) , }
};
}
