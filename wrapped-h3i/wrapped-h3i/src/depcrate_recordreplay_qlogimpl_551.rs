// Generated macro for impl_551 (impl)
macro_rules! Depcrate_recordreplay_qlogimpl_551 {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"impl_551"}
// Dependencies: {}
impl From < JsonEvent > for H3Actions { fn from (event : JsonEvent) -> Self { let mut actions = vec ! [] ; match event . name . as_ref () { "h3i:wait" => { let wait_type = serde_json :: from_value :: < WaitType > (event . clone () . data) ; if let Ok (wt) = wait_type { actions . push (Action :: Wait { wait_type : wt }) ; } else { log :: debug ! ("couldn't create action from event: {event:?}") ; } } , _ => unimplemented ! () , } Self (actions) } }
};
}
