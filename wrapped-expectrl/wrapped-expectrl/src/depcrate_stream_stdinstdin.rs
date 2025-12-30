// Generated macro for Stdin (struct)
macro_rules! Depcrate_stream_stdinStdin {
() => {
// Module: crate::stream::stdin
// Provides: {"Stdin"}
// Dependencies: {}
# [doc = " A non blocking version of STDIN."] # [doc = ""] # [doc = " It's not recomended to be used directly."] # [doc = " But we expose it because it cab be used with [`Session::interact`]."] # [doc = ""] # [doc = " [`Session::interact`]: crate::session::Session::interact"] # [derive (Debug)] pub struct Stdin { inner : inner :: StdinInner , }
};
}
