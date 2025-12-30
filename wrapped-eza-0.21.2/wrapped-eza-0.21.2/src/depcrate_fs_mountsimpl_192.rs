// Generated macro for impl_192 (impl)
macro_rules! Depcrate_fs_mountsimpl_192 {
() => {
// Module: crate::fs::mounts
// Provides: {"impl_192"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { # [allow (unreachable_patterns)] match self { # [cfg (target_os = "macos")] Error :: GetFSStatError (err) => write ! (f , "getfsstat failed: {err}") , # [cfg (target_os = "linux")] Error :: IOError (err) => write ! (f , "failed to read /proc/mounts: {err}") , _ => write ! (f , "Unknown error") , } } }
};
}
