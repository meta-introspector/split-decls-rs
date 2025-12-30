// Generated macro for impl_72 (impl)
macro_rules! Depcrate_parseimpl_72 {
() => {
// Module: crate::parse
// Provides: {"impl_72"}
// Dependencies: {}
impl Parser < '_ > { # [doc = " Strip leading qualifiers, if any."] pub (crate) fn strip_leading_qualifiers (& mut self) { # [allow (clippy :: byte_char_slices)] const QUALIFIERS : & [u8] = & [b'r' , b'n' , b'N' , b'o' , b'O' , b'R' , b'V' ,] ; self . consume_while (| b | QUALIFIERS . contains (& b)) ; } # [doc = " Chomp until we hit a non-digit."] # [doc = ""] # [doc = " + and - prefixes are not supported."] fn chomp_digits (& mut self) -> Result < & str > { let old_split_point = self . split_point ; if ! self . peek () ? . is_ascii_digit () { return Err (ErrorKind :: ExpectedInteger) ; } self . consume_while (| b | b . is_ascii_digit ()) ; Ok (& self . data [old_split_point .. self . split_point]) } fn parse_u64 (& mut self) -> Result < u64 > { self . chomp_digits () ? . parse () . map_err (| _ | ErrorKind :: IntegerTooLarge) } fn parse_u8 (& mut self) -> Result < u8 > { self . chomp_digits () ? . parse () . map_err (| _ | ErrorKind :: IntegerTooLarge) } }
};
}
