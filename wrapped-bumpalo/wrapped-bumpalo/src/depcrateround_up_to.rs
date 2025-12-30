// Generated macro for round_up_to (function)
macro_rules! Depcrateround_up_to {
() => {
// Module: crate
// Provides: {"round_up_to"}
// Dependencies: {}
# [inline] pub (crate) const fn round_up_to (n : usize , divisor : usize) -> Option < usize > { debug_assert ! (divisor > 0) ; debug_assert ! (divisor . is_power_of_two ()) ; match n . checked_add (divisor - 1) { Some (x) => Some (x & ! (divisor - 1)) , None => None , } }
};
}
