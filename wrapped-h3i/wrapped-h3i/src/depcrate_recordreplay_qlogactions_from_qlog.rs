// Generated macro for actions_from_qlog (function)
macro_rules! Depcrate_recordreplay_qlogactions_from_qlog {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"actions_from_qlog"}
// Dependencies: {}
pub fn actions_from_qlog (event : Event , host_override : Option < & str >) -> H3Actions { let mut actions = vec ! [] ; match & event . data { EventData :: PacketSent (ps) => { let packet_actions : H3Actions = ps . into () ; actions . extend (packet_actions . 0) ; } , EventData :: H3FrameCreated (fc) => { let mut frame_created = H3FrameCreatedEx { frame_created : fc . clone () , ex_data : event . ex_data . clone () , } ; if let Some (host) = host_override { frame_created . ex_data . insert ("host_override" . into () , host . into ()) ; } actions . push (frame_created . into ()) ; } , EventData :: H3StreamTypeSet (st) => { let stream_actions = from_qlog_stream_type_set (st , & event . ex_data) ; actions . extend (stream_actions) ; } , _ => () , } H3Actions (actions) }
};
}
