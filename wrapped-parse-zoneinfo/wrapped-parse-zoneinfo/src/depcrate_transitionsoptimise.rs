// Generated macro for optimise (function)
macro_rules! Depcrate_transitionsoptimise {
() => {
// Module: crate::transitions
// Provides: {"optimise"}
// Dependencies: {}
# [allow (unused_results)] fn optimise (transitions : & mut FixedTimespanSet) { let mut from_i = 0 ; let mut to_i = 0 ; while from_i < transitions . rest . len () { if to_i > 1 { let from = transitions . rest [from_i] . 0 ; let to = transitions . rest [to_i - 1] . 0 ; if from + transitions . rest [to_i - 1] . 1 . total_offset () <= to + transitions . rest [to_i - 2] . 1 . total_offset () { transitions . rest [to_i - 1] . 1 = transitions . rest [from_i] . 1 . clone () ; from_i += 1 ; continue ; } } if to_i == 0 || transitions . rest [to_i - 1] . 1 != transitions . rest [from_i] . 1 { transitions . rest [to_i] = transitions . rest [from_i] . clone () ; to_i += 1 ; } from_i += 1 } transitions . rest . truncate (to_i) ; if ! transitions . rest . is_empty () && transitions . first == transitions . rest [0] . 1 { transitions . rest . remove (0) ; } }
};
}
