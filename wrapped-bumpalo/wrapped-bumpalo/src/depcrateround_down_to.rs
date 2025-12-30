// Generated macro for round_down_to (function)
macro_rules! Depcrateround_down_to {
() => {
// Module: crate
// Provides: {"round_down_to"}
// Dependencies: {}
# [inline] pub (crate) fn round_down_to (n : usize , divisor : usize) -> usize { debug_assert ! (divisor > 0) ; debug_assert ! (divisor . is_power_of_two ()) ; n & ! (divisor - 1) }
};
}
