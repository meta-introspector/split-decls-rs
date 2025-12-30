// Generated macro for unescape_attribute_values (function)
macro_rules! Depcrate_parseunescape_attribute_values {
() => {
// Module: crate::parse
// Provides: {"unescape_attribute_values"}
// Dependencies: {}
fn unescape_attribute_values (input : & BStr) -> Result < Cow < '_ , BStr > , Error > { if ! input . contains (& b'=') { return Ok (Cow :: Borrowed (input)) ; } let mut out : Cow < '_ , BStr > = Cow :: Borrowed ("" . into ()) ; for attr in input . split (| & c | c == b' ') { let split_point = attr . find_byte (b'=') . map_or_else (| | attr . len () , | i | i + 1) ; let (name , value) = attr . split_at (split_point) ; if value . contains (& b'\\') { let out = out . to_mut () ; out . push_str (name) ; out . push_str (unescape_and_check_attr_value (value . into ()) ?) ; out . push (b' ') ; } else { check_attribute_value (value . as_bstr ()) ? ; match out { Cow :: Borrowed (_) => { let end = out . len () + attr . len () + 1 ; out = Cow :: Borrowed (& input [0 .. end . min (input . len ())]) ; } Cow :: Owned (_) => { let out = out . to_mut () ; out . push_str (name) ; out . push_str (value) ; out . push (b' ') ; } } } } Ok (out) }
};
}
