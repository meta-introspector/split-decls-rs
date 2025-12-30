// Generated macro for handle_fin (function)
macro_rules! Depcrate_clienthandle_fin {
() => {
// Module: crate::client
// Provides: {"handle_fin"}
// Dependencies: {}
fn handle_fin (responded_streams : & mut Vec < StreamEvent > , stream_parsers : & mut StreamParserMap , stream_id : u64 ,) { responded_streams . push (StreamEvent { stream_id , event_type : StreamEventType :: Finished , }) ; stream_parsers . remove (& stream_id) ; }
};
}
