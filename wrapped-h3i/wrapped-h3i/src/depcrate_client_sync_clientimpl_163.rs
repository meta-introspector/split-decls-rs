// Generated macro for impl_163 (impl)
macro_rules! Depcrate_client_sync_clientimpl_163 {
() => {
// Module: crate::client::sync_client
// Provides: {"impl_163"}
// Dependencies: {}
impl Client for SyncClient { fn stream_parsers_mut (& mut self) -> & mut StreamParserMap { & mut self . stream_parsers } fn handle_response_frame (& mut self , stream_id : u64 , frame : H3iFrame) { self . streams . insert (stream_id , frame) ; } }
};
}
