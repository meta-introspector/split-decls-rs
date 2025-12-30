// Generated macro for range (function)
macro_rules! Depcrate_rt_atomicrange {
() => {
// Module: crate::rt::atomic
// Provides: {"range"}
// Dependencies: {}
fn range (cnt : u16) -> (usize , usize) { let start = index (cnt . saturating_sub (MAX_ATOMIC_HISTORY as u16)) ; let mut end = index (cmp :: min (cnt , MAX_ATOMIC_HISTORY as u16)) ; if end == 0 { end = MAX_ATOMIC_HISTORY ; } assert ! (start <= end , "[loom internal bug] cnt = {}; start = {}; end = {}" , cnt , start , end) ; (start , end) }
};
}
