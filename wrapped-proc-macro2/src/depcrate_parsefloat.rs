// Generated macro for float (function)
macro_rules! Depcrate_parsefloat {
() => {
// Module: crate::parse
// Provides: {"float"}
// Dependencies: {}
fn float (input : Cursor) -> Result < Cursor , Reject > { let mut rest = float_digits (input) ? ; if let Some (ch) = rest . chars () . next () { if is_ident_start (ch) { rest = ident_not_raw (rest) ? . 0 ; } } word_break (rest) }
};
}
