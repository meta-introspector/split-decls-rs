// Generated macro for MioStream (enum)
macro_rules! Depcrate_socketMioStream {
() => {
// Module: crate::socket
// Provides: {"MioStream"}
// Dependencies: {}
# [derive (Debug)] pub enum MioStream { Tcp (mio :: net :: TcpStream) , # [cfg (unix)] Uds (mio :: net :: UnixStream) , }
};
}
