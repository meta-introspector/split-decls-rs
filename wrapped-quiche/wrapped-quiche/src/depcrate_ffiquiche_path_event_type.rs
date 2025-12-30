// Generated macro for quiche_path_event_type (function)
macro_rules! Depcrate_ffiquiche_path_event_type {
() => {
// Module: crate::ffi
// Provides: {"quiche_path_event_type"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_path_event_type (ev : & PathEvent) -> u32 { match ev { PathEvent :: New { .. } => 0 , PathEvent :: Validated { .. } => 1 , PathEvent :: FailedValidation { .. } => 2 , PathEvent :: Closed { .. } => 3 , PathEvent :: ReusedSourceConnectionId { .. } => 4 , PathEvent :: PeerMigrated { .. } => 5 , } }
};
}
