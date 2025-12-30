// Generated macro for Http2SessionRecvHeadersParams (struct)
macro_rules! Depcrate_h2Http2SessionRecvHeadersParams {
() => {
// Module: crate::h2
// Provides: {"Http2SessionRecvHeadersParams"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default)] pub struct Http2SessionRecvHeadersParams { pub stream_id : u32 , pub headers : Vec < String > , pub fin : bool , }
};
}
