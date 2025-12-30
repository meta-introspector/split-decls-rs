// Generated macro for PathStats (struct)
macro_rules! Depcrate_connection_statsPathStats {
() => {
// Module: crate::connection::stats
// Provides: {"PathStats"}
// Dependencies: {}
# [doc = " Statistics related to a transmission path"] # [derive (Debug , Default , Copy , Clone)] # [non_exhaustive] pub struct PathStats { # [doc = " Current best estimate of this connection's latency (round-trip-time)"] pub rtt : Duration , # [doc = " Current congestion window of the connection"] pub cwnd : u64 , # [doc = " Congestion events on the connection"] pub congestion_events : u64 , # [doc = " The amount of packets lost on this path"] pub lost_packets : u64 , # [doc = " The amount of bytes lost on this path"] pub lost_bytes : u64 , # [doc = " The amount of packets sent on this path"] pub sent_packets : u64 , # [doc = " The amount of PLPMTUD probe packets sent on this path (also counted by `sent_packets`)"] pub sent_plpmtud_probes : u64 , # [doc = " The amount of PLPMTUD probe packets lost on this path (ignored by `lost_packets` and"] # [doc = " `lost_bytes`)"] pub lost_plpmtud_probes : u64 , # [doc = " The number of times a black hole was detected in the path"] pub black_holes_detected : u64 , # [doc = " Largest UDP payload size the path currently supports"] pub current_mtu : u16 , }
};
}
