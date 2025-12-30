// Generated macro for Connection (struct)
macro_rules! Depcrate_client_git_async_ioConnection {
() => {
// Module: crate::client::git::async_io
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " A TCP connection to either a `git` daemon or a spawned `git` process."] # [doc = ""] # [doc = " When connecting to a daemon, additional context information is sent with the first line of the handshake."] pub struct Connection < R , W > { pub (in crate :: client) writer : W , pub (in crate :: client) line_provider : StreamingPeekableIter < R > , pub (in crate :: client) state : ConnectionState , }
};
}
