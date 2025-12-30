// Generated macro for ConnectionStats (struct)
macro_rules! Depcrate_connection_statsConnectionStats {
() => {
// Module: crate::connection::stats
// Provides: {"ConnectionStats"}
// Dependencies: {}
# [doc = " Connection statistics"] # [derive (Debug , Default , Copy , Clone)] # [non_exhaustive] pub struct ConnectionStats { # [doc = " Statistics about UDP datagrams transmitted on a connection"] pub udp_tx : UdpStats , # [doc = " Statistics about UDP datagrams received on a connection"] pub udp_rx : UdpStats , # [doc = " Statistics about frames transmitted on a connection"] pub frame_tx : FrameStats , # [doc = " Statistics about frames received on a connection"] pub frame_rx : FrameStats , # [doc = " Statistics related to the current transmission path"] pub path : PathStats , }
};
}
