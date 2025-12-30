// Generated macro for merge_intervals (function)
macro_rules! Depcrate_ipnetmerge_intervals {
() => {
// Module: crate::ipnet
// Provides: {"merge_intervals"}
// Dependencies: {}
fn merge_intervals < T : Copy + Ord > (mut intervals : Vec < (T , T) >) -> Vec < (T , T) > { if intervals . len () == 0 { return intervals ; } intervals . sort () ; let mut res : Vec < (T , T) > = Vec :: new () ; let (mut start , mut end) = intervals [0] ; let mut i = 1 ; let len = intervals . len () ; while i < len { let (next_start , next_end) = intervals [i] ; if end >= next_start { start = min (start , next_start) ; end = max (end , next_end) ; } else { res . push ((start , end)) ; start = next_start ; end = next_end ; } i += 1 ; } res . push ((start , end)) ; res }
};
}
