// Generated macro for UdpStats (struct)
macro_rules! Depcrate_connection_statsUdpStats {
() => {
// Module: crate::connection::stats
// Provides: {"UdpStats"}
// Dependencies: {}
# [doc = " Statistics about UDP datagrams transmitted or received on a connection"] # [doc = ""] # [doc = " All QUIC packets are carried by UDP datagrams. Hence, these statistics cover all traffic on a connection."] # [derive (Default , Debug , Copy , Clone)] # [non_exhaustive] pub struct UdpStats { # [doc = " The amount of UDP datagrams observed"] pub datagrams : u64 , # [doc = " The total amount of bytes which have been transferred inside UDP datagrams"] pub bytes : u64 , # [doc = " The amount of I/O operations executed"] # [doc = ""] # [doc = " Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use."] pub ios : u64 , }
};
}
