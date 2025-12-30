// Generated macro for parse_time_zone (function)
macro_rules! Depcrate_parsers_timezoneparse_time_zone {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_time_zone"}
// Dependencies: {}
# [doc = " Parses the [`TimeZoneIdentifier`][tz] node."] # [doc = ""] # [doc = " [tz]: https://tc39.es/proposal-temporal/#prod-TimeZoneIdentifier"] pub (crate) fn parse_time_zone < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < TimeZoneRecord < 'a , T > > { let is_iana = cursor . check (is_tz_leading_char) ? . ok_or (ParseError :: abrupt_end ("TimeZoneAnnotation")) ? ; let is_offset = cursor . check_or (false , is_ascii_sign) ? ; if is_iana { return Ok (TimeZoneRecord :: Name (parse_tz_iana_name (cursor) ?)) ; } else if is_offset { let offset = parse_utc_offset_minute_precision_strict (cursor) ? ; return Ok (TimeZoneRecord :: Offset (offset)) ; } Err (ParseError :: TzLeadingChar) }
};
}
