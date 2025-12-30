// Generated macro for ConnectionSummary (struct)
macro_rules! Depcrate_client_connection_summaryConnectionSummary {
() => {
// Module: crate::client::connection_summary
// Provides: {"ConnectionSummary"}
// Dependencies: {}
# [doc = " A summary of all frames received on a connection. There are some extra"] # [doc = " fields included to provide additional context into the connection's"] # [doc = " behavior."] # [doc = ""] # [doc = " ConnectionSummary implements [Serialize]. HTTP/3 frames that contain binary"] # [doc = " payload are serialized using the qlog"] # [doc = " [hexstring](https://www.ietf.org/archive/id/draft-ietf-quic-qlog-main-schema-10.html#section-1.2)"] # [doc = " format - \"an even-length lowercase string of hexadecimally encoded bytes"] # [doc = " examples: 82dc, 027339, 4cdbfd9bf0\""] # [derive (Default , Debug)] pub struct ConnectionSummary { pub stream_map : StreamMap , # [doc = " L4 statistics received from the connection."] pub stats : Option < Stats > , # [doc = " Statistics about all paths of the connection."] pub path_stats : Vec < PathStats > , # [doc = " Details about why the connection closed."] pub conn_close_details : ConnectionCloseDetails , }
};
}
