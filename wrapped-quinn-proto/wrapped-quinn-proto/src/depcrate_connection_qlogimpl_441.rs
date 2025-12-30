// Generated macro for impl_441 (impl)
macro_rules! Depcrate_connection_qlogimpl_441 {
() => {
// Module: crate::connection::qlog
// Provides: {"impl_441"}
// Dependencies: {}
# [cfg (feature = "qlog")] impl QlogStream { fn emit_event (& self , orig_rem_cid : ConnectionId , event : EventData , now : Instant) { let mut event = Event :: with_time (0.0 , event) ; event . group_id = Some (orig_rem_cid . to_string ()) ; let mut qlog_streamer = self . 0 . lock () . unwrap () ; if let Err (e) = qlog_streamer . add_event_with_instant (event , now) { warn ! ("could not emit qlog event: {e}") ; } } }
};
}
