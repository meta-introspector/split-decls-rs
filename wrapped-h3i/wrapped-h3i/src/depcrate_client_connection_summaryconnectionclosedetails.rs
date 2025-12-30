// Generated macro for ConnectionCloseDetails (struct)
macro_rules! Depcrate_client_connection_summaryConnectionCloseDetails {
() => {
// Module: crate::client::connection_summary
// Provides: {"ConnectionCloseDetails"}
// Dependencies: {}
# [doc = " Denotes why the connection was closed."] # [derive (Default)] pub struct ConnectionCloseDetails { peer_error : Option < ConnectionError > , local_error : Option < ConnectionError > , # [doc = " If the connection timed out."] pub timed_out : bool , # [doc = " Return the session from the underlying connection."] pub session : Option < Vec < u8 > > , }
};
}
