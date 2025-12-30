// Generated macro for Endpoint (struct)
macro_rules! Depcrate_endpointEndpoint {
() => {
// Module: crate::endpoint
// Provides: {"Endpoint"}
// Dependencies: {}
# [doc = " The main entry point to the library"] # [doc = ""] # [doc = " This object performs no I/O whatsoever. Instead, it consumes incoming packets and"] # [doc = " connection-generated events via `handle` and `handle_event`."] pub struct Endpoint { rng : StdRng , index : ConnectionIndex , connections : Slab < ConnectionMeta > , local_cid_generator : Box < dyn ConnectionIdGenerator > , config : Arc < EndpointConfig > , server_config : Option < Arc < ServerConfig > > , # [doc = " Whether the underlying UDP socket promises not to fragment packets"] allow_mtud : bool , # [doc = " Time at which a stateless reset was most recently sent"] last_stateless_reset : Option < Instant > , # [doc = " Buffered Initial and 0-RTT messages for pending incoming connections"] incoming_buffers : Slab < IncomingBuffer > , all_incoming_buffers_total_bytes : u64 , }
};
}
