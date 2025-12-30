// Generated macro for impl_944 (impl)
macro_rules! Depcrate_http_websocketimpl_944 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_944"}
// Dependencies: {}
impl std :: str :: FromStr for Protocols { type Err = Error ; fn from_str (protocol : & str) -> Result < Self , Self :: Err > { if protocol . eq_ignore_ascii_case ("graphql-ws") { Ok (Protocols :: SubscriptionsTransportWS) } else if protocol . eq_ignore_ascii_case ("graphql-transport-ws") { Ok (Protocols :: GraphQLWS) } else { Err (Error :: new (format ! ("Unsupported Sec-WebSocket-Protocol: {}" , protocol))) } } }
};
}
