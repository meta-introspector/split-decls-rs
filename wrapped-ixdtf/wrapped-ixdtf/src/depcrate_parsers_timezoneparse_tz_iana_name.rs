// Generated macro for parse_tz_iana_name (function)
macro_rules! Depcrate_parsers_timezoneparse_tz_iana_name {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_tz_iana_name"}
// Dependencies: {}
# [doc = " Parse a `TimeZoneIANAName` Parse Node"] pub (crate) fn parse_tz_iana_name < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < & 'a [T :: CodeUnit] > { assert_syntax ! (cursor . check_or (false , is_tz_leading_char) ?, TzLeadingChar) ; let tz_name_start = cursor . pos () ; while let Some (potential_value_char) = cursor . next () ? { if cursor . check_or (true , is_annotation_close) ? { break ; } if is_tz_name_separator (potential_value_char) { assert_syntax ! (cursor . check_or (false , is_tz_char) ?, IanaCharPostSeparator ,) ; continue ; } assert_syntax ! (is_tz_char (potential_value_char) , IanaChar ,) ; } cursor . slice (tz_name_start , cursor . pos ()) . ok_or (ParseError :: ImplAssert) }
};
}
