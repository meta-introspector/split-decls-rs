// Generated macro for HttpTransactionQuicSendRequestHeadersParams (struct)
macro_rules! Depcrate_httpHttpTransactionQuicSendRequestHeadersParams {
() => {
// Module: crate::http
// Provides: {"HttpTransactionQuicSendRequestHeadersParams"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default)] pub struct HttpTransactionQuicSendRequestHeadersParams { pub headers : Vec < String > , pub quic_priority_incremental : bool , pub quic_priority_type : String , pub quic_priority_urgency : u8 , pub quic_stream_id : u64 , }
};
}
