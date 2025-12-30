// Generated macro for handle_qlog (function)
macro_rules! Depcrate_clienthandle_qlog {
() => {
// Module: crate::client
// Provides: {"handle_qlog"}
// Dependencies: {}
fn handle_qlog (qlog_streamer : Option < & mut QlogStreamer > , qlog_frame : Http3Frame , stream_id : u64 ,) { if let Some (s) = qlog_streamer { let ev_data = EventData :: H3FrameParsed (H3FrameParsed { stream_id , frame : qlog_frame , .. Default :: default () }) ; s . add_event_data_now (ev_data) . ok () ; } }
};
}
