// Generated macro for ConnectionError (enum)
macro_rules! Depcrate_connectionConnectionError {
() => {
// Module: crate::connection
// Provides: {"ConnectionError"}
// Dependencies: {}
# [doc = " Reasons why a connection might be lost"] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum ConnectionError { # [doc = " The peer doesn't implement any supported version"] # [error ("peer doesn't implement any supported version")] VersionMismatch , # [doc = " The peer violated the QUIC specification as understood by this implementation"] # [error (transparent)] TransportError (# [from] TransportError) , # [doc = " The peer's QUIC stack aborted the connection automatically"] # [error ("aborted by peer: {0}")] ConnectionClosed (frame :: ConnectionClose) , # [doc = " The peer closed the connection"] # [error ("closed by peer: {0}")] ApplicationClosed (frame :: ApplicationClose) , # [doc = " The peer is unable to continue processing this connection, usually due to having restarted"] # [error ("reset by peer")] Reset , # [doc = " Communication with the peer has lapsed for longer than the negotiated idle timeout"] # [doc = ""] # [doc = " If neither side is sending keep-alives, a connection will time out after a long enough idle"] # [doc = " period even if the peer is still reachable. See also [`TransportConfig::max_idle_timeout()`]"] # [doc = " and [`TransportConfig::keep_alive_interval()`]."] # [error ("timed out")] TimedOut , # [doc = " The local application closed the connection"] # [error ("closed")] LocallyClosed , # [doc = " The connection could not be created because not enough of the CID space is available"] # [doc = ""] # [doc = " Try using longer connection IDs."] # [error ("CIDs exhausted")] CidsExhausted , }
};
}
