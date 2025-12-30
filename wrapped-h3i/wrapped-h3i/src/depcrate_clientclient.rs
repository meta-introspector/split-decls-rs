// Generated macro for Client (trait)
macro_rules! Depcrate_clientClient {
() => {
// Module: crate::client
// Provides: {"Client"}
// Dependencies: {}
pub (crate) trait Client { # [doc = " Gives mutable access to the stream parsers to update their state."] fn stream_parsers_mut (& mut self) -> & mut StreamParserMap ; # [doc = " Handles a response frame. This allows [`Client`]s to customize how they"] # [doc = " construct a [`StreamMap`] from a list of frames."] fn handle_response_frame (& mut self , stream_id : u64 , frame : H3iFrame) ; }
};
}
