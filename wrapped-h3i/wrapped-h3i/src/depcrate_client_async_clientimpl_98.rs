// Generated macro for impl_98 (impl)
macro_rules! Depcrate_client_async_clientimpl_98 {
() => {
// Module: crate::client::async_client
// Provides: {"impl_98"}
// Dependencies: {}
impl Client for H3iDriver { fn stream_parsers_mut (& mut self) -> & mut StreamParserMap { & mut self . stream_parsers } fn handle_response_frame (& mut self , stream_id : u64 , frame : crate :: frame :: H3iFrame ,) { self . record_tx . send (ConnectionRecord :: StreamedFrame { stream_id , frame }) . expect ("H3iDriver task dropped") } }
};
}
