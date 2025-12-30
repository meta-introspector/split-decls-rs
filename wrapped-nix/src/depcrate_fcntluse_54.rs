// Generated macro for use_54 (pub_use)
macro_rules! Depcrate_fcntluse_54 {
() => {
// Module: crate::fcntl
// Provides: {"use_54"}
// Dependencies: {}
# [cfg (any (linux_android , target_os = "emscripten" , target_os = "fuchsia" , target_os = "wasi" , target_env = "uclibc" , target_os = "freebsd"))] # [cfg (feature = "fs")] pub use self :: posix_fadvise :: { posix_fadvise , PosixFadviseAdvice } ;
};
}
