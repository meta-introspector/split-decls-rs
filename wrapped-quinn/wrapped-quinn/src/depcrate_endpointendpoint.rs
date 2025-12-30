// Generated macro for Endpoint (struct)
macro_rules! Depcrate_endpointEndpoint {
() => {
// Module: crate::endpoint
// Provides: {"Endpoint"}
// Dependencies: {}
# [doc = " A QUIC endpoint."] # [doc = ""] # [doc = " An endpoint corresponds to a single UDP socket, may host many connections, and may act as both"] # [doc = " client and server for different connections."] # [doc = ""] # [doc = " May be cloned to obtain another handle to the same endpoint."] # [derive (Debug , Clone)] pub struct Endpoint { pub (crate) inner : EndpointRef , pub (crate) default_client_config : Option < ClientConfig > , runtime : Arc < dyn Runtime > , }
};
}
