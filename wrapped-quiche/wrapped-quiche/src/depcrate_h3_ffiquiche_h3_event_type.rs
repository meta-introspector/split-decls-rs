// Generated macro for quiche_h3_event_type (function)
macro_rules! Depcrate_h3_ffiquiche_h3_event_type {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_event_type"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_event_type (ev : & h3 :: Event) -> u32 { match ev { h3 :: Event :: Headers { .. } => 0 , h3 :: Event :: Data => 1 , h3 :: Event :: Finished => 2 , h3 :: Event :: GoAway => 3 , h3 :: Event :: Reset { .. } => 4 , h3 :: Event :: PriorityUpdate => 5 , } }
};
}
