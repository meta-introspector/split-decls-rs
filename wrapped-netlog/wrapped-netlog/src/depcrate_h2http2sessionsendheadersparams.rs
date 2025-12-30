// Generated macro for Http2SessionSendHeadersParams (struct)
macro_rules! Depcrate_h2Http2SessionSendHeadersParams {
() => {
// Module: crate::h2
// Provides: {"Http2SessionSendHeadersParams"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default)] pub struct Http2SessionSendHeadersParams { pub stream_id : u32 , pub headers : Vec < String > , pub fin : bool , pub has_priority : bool , pub exclusive : bool , pub weight : u16 , pub parent_stream_id : u32 , pub source_dependency : SourceDependency , }
};
}
