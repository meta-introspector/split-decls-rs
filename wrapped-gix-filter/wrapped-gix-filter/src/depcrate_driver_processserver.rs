// Generated macro for Server (struct)
macro_rules! Depcrate_driver_processServer {
() => {
// Module: crate::driver::process
// Provides: {"Server"}
// Dependencies: {}
# [doc = " A handle to facilitate typical server interactions that include the handshake and command-invocations."] pub struct Server { # [doc = " The names of the capabilities we can expect the client to use."] capabilities : Capabilities , # [doc = " The negotiated version of the protocol, it's the highest supported one."] version : usize , # [doc = " A way to receive information from the client."] input : StreamingPeekableIter < std :: io :: StdinLock < 'static > > , # [doc = " A way to send information to the client."] out : Writer < std :: io :: StdoutLock < 'static > > , }
};
}
