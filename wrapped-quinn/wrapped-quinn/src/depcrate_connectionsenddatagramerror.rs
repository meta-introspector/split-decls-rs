// Generated macro for SendDatagramError (enum)
macro_rules! Depcrate_connectionSendDatagramError {
() => {
// Module: crate::connection
// Provides: {"SendDatagramError"}
// Dependencies: {}
# [doc = " Errors that can arise when sending a datagram"] # [derive (Debug , Error , Clone , Eq , PartialEq)] pub enum SendDatagramError { # [doc = " The peer does not support receiving datagram frames"] # [error ("datagrams not supported by peer")] UnsupportedByPeer , # [doc = " Datagram support is disabled locally"] # [error ("datagram support disabled")] Disabled , # [doc = " The datagram is larger than the connection can currently accommodate"] # [doc = ""] # [doc = " Indicates that the path MTU minus overhead or the limit advertised by the peer has been"] # [doc = " exceeded."] # [error ("datagram too large")] TooLarge , # [doc = " The connection was lost"] # [error ("connection lost")] ConnectionLost (# [from] ConnectionError) , }
};
}
