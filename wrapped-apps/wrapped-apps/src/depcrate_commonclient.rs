// Generated macro for Client (struct)
macro_rules! Depcrate_commonClient {
() => {
// Module: crate::common
// Provides: {"Client"}
// Dependencies: {}
pub struct Client { pub conn : quiche :: Connection , pub http_conn : Option < Box < dyn HttpConn > > , pub client_id : ClientId , pub app_proto_selected : bool , pub partial_requests : std :: collections :: HashMap < u64 , PartialRequest > , pub partial_responses : std :: collections :: HashMap < u64 , PartialResponse > , pub max_datagram_size : usize , pub loss_rate : f64 , pub max_send_burst : usize , }
};
}
