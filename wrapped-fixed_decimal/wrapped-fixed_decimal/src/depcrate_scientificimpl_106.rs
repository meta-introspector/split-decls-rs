// Generated macro for impl_106 (impl)
macro_rules! Depcrate_scientificimpl_106 {
() => {
// Module: crate::scientific
// Provides: {"impl_106"}
// Dependencies: {}
impl ScientificDecimal { # [inline] pub fn try_from_str (s : & str) -> Result < Self , ParseError > { Self :: try_from_utf8 (s . as_bytes ()) } pub fn try_from_utf8 (code_units : & [u8]) -> Result < Self , ParseError > { if code_units . contains (& b'E') { return Err (ParseError :: Syntax) ; } let mut parts = code_units . split (| & c | c == b'e') ; let significand = parts . next () . ok_or (ParseError :: Syntax) ? ; let exponent = parts . next () . ok_or (ParseError :: Syntax) ? ; if parts . next () . is_some () { return Err (ParseError :: Syntax) ; } Ok (ScientificDecimal :: from (Decimal :: try_from_utf8 (significand) ? , FixedInteger :: try_from_utf8 (exponent) ? ,)) } }
};
}
