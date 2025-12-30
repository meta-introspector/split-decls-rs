// Generated macro for parse_fraction (function)
macro_rules! Depcrate_parsers_timeparse_fraction {
() => {
// Module: crate::parsers::time
// Provides: {"parse_fraction"}
// Dependencies: {}
# [doc = " Parse a `Fraction` value"] # [doc = ""] # [doc = " This is primarily used in ISO8601 to add percision past"] # [doc = " a second."] # [inline] pub (crate) fn parse_fraction < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < Option < Fraction > > { if ! cursor . check_or (false , is_decimal_separator) ? { return Ok (None) ; } cursor . next_or (ParseError :: FractionPart) ? ; let mut value = 0 ; let mut digits : u8 = 0 ; while cursor . check_or (false , | ch | ch . is_ascii_digit ()) ? { let next_value = u64 :: from (cursor . next_digit () ? . ok_or (ParseError :: ImplAssert) ?) ; if digits < 18 { value = value * 10 + next_value ; } digits = digits . saturating_add (1) ; } let digits = NonZeroU8 :: new (digits) . ok_or (ParseError :: FractionPart) ? ; Ok (Some (Fraction { digits , value })) }
};
}
