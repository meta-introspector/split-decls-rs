// Generated macro for bwr_false (function)
macro_rules! Depcrate_arbitrary__std_syncbwr_false {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"bwr_false"}
// Dependencies: {}
fn bwr_false () -> BarrierWaitResult { let barrier = Arc :: new (Barrier :: new (2)) ; let b2 = barrier . clone () ; let jh = thread :: spawn (move | | b2 . wait ()) ; let bwr1 = barrier . wait () ; let bwr2 = jh . join () . unwrap () ; if bwr1 . is_leader () { bwr2 } else { bwr1 } }
};
}
