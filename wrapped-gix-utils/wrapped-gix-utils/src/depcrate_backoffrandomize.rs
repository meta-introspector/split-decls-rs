// Generated macro for randomize (function)
macro_rules! Depcrate_backoffrandomize {
() => {
// Module: crate::backoff
// Provides: {"randomize"}
// Dependencies: {}
fn randomize (backoff_ms : usize) -> usize { let new_value = (fastrand :: usize (750 ..= 1250) * backoff_ms) / 1000 ; if new_value == 0 { backoff_ms } else { new_value } }
};
}
