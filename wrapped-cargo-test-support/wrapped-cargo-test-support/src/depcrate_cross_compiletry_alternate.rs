// Generated macro for try_alternate (function)
macro_rules! Depcrate_cross_compiletry_alternate {
() => {
// Module: crate::cross_compile
// Provides: {"try_alternate"}
// Dependencies: {}
# [doc = " A possible alternate target-triple to build with."] pub (crate) fn try_alternate () -> Option < & 'static str > { if cfg ! (target_os = "macos") { Some ("x86_64-apple-darwin") } else if cfg ! (target_os = "linux") { Some ("i686-unknown-linux-gnu") } else if cfg ! (all (target_os = "windows" , target_env = "msvc")) { Some ("i686-pc-windows-msvc") } else if cfg ! (all (target_os = "windows" , target_env = "gnu")) { Some ("i686-pc-windows-gnu") } else { None } }
};
}
