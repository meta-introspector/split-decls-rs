// Generated macro for is_old_enough_to_be_collected (function)
macro_rules! Depcrate_persist_fsis_old_enough_to_be_collected {
() => {
// Module: crate::persist::fs
// Provides: {"is_old_enough_to_be_collected"}
// Dependencies: {}
fn is_old_enough_to_be_collected (timestamp : SystemTime) -> bool { timestamp < SystemTime :: now () - Duration :: from_secs (10) }
};
}
