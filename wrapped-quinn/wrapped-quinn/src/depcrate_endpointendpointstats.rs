// Generated macro for EndpointStats (struct)
macro_rules! Depcrate_endpointEndpointStats {
() => {
// Module: crate::endpoint
// Provides: {"EndpointStats"}
// Dependencies: {}
# [doc = " Statistics on [Endpoint] activity"] # [non_exhaustive] # [derive (Debug , Default , Copy , Clone)] pub struct EndpointStats { # [doc = " Cummulative number of Quic handshakes accepted by this [Endpoint]"] pub accepted_handshakes : u64 , # [doc = " Cummulative number of Quic handshakees sent from this [Endpoint]"] pub outgoing_handshakes : u64 , # [doc = " Cummulative number of Quic handshakes refused on this [Endpoint]"] pub refused_handshakes : u64 , # [doc = " Cummulative number of Quic handshakes ignored on this [Endpoint]"] pub ignored_handshakes : u64 , }
};
}
