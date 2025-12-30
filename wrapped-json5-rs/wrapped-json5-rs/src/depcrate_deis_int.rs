// Generated macro for is_int (function)
macro_rules! Depcrate_deis_int {
() => {
// Module: crate::de
// Provides: {"is_int"}
// Dependencies: {}
fn is_int (s : & str) -> bool { ! s . contains ('.') && (is_hex_literal (s) || (! s . contains ('e') && ! s . contains ('E'))) && ! is_infinite (s) && ! is_nan (s) }
};
}
