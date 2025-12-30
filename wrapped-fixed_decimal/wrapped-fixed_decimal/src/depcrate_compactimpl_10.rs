// Generated macro for impl_10 (impl)
macro_rules! Depcrate_compactimpl_10 {
() => {
// Module: crate::compact
// Provides: {"impl_10"}
// Dependencies: {}
impl CompactDecimal { # [inline] # [doc = " Parses a [`CompactDecimal`]."] pub fn try_from_str (s : & str) -> Result < Self , ParseError > { Self :: try_from_utf8 (s . as_bytes ()) } # [doc = " The deprecated letter e is not accepted as a synonym for c."] fn try_from_utf8 (code_units : & [u8]) -> Result < Self , ParseError > { if code_units . iter () . any (| & c | c == b'e' || c == b'E') { return Err (ParseError :: Syntax) ; } let mut parts = code_units . split (| & c | c == b'c') ; let significand = Decimal :: try_from_utf8 (parts . next () . ok_or (ParseError :: Syntax) ?) ? ; match parts . next () { None => Ok (CompactDecimal { significand , exponent : 0 , }) , Some (exponent_str) => { let exponent_str = core :: str :: from_utf8 (exponent_str) . map_err (| _ | ParseError :: Syntax) ? ; if parts . next () . is_some () { return Err (ParseError :: Syntax) ; } if exponent_str . is_empty () || exponent_str . bytes () . next () == Some (b'0') || ! exponent_str . bytes () . all (| c | c . is_ascii_digit ()) { return Err (ParseError :: Syntax) ; } let exponent = exponent_str . parse () . map_err (| _ | ParseError :: Limit) ? ; Ok (CompactDecimal { significand , exponent , }) } } } }
};
}
