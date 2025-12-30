// Generated macro for div_ceil (function)
macro_rules! Depcrate_nfa_thompson_backtrackdiv_ceil {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"div_ceil"}
// Dependencies: {}
# [doc = " Integer division, but rounds up instead of down."] fn div_ceil (lhs : usize , rhs : usize) -> usize { if lhs % rhs == 0 { lhs / rhs } else { (lhs / rhs) + 1 } }
};
}
