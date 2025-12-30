// Generated macro for parse_ambiguous_tz_annotation (function)
macro_rules! Depcrate_parsers_timezoneparse_ambiguous_tz_annotation {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_ambiguous_tz_annotation"}
// Dependencies: {}
# [doc = " We support two kinds of annotations here: annotations (e.g. `[u-ca=foo]`)"] # [doc = " and \"time zone annotations\" (`[UTC]` or `[+05:30]`)"] # [doc = ""] # [doc = " When parsing bracketed contents, we need to figure out which one we're dealing with."] # [doc = ""] # [doc = " This function returns a time zone annotation if we are dealing with a time zone,"] # [doc = " otherwise it returns None (and the caller must handle non-tz annotations)."] pub (crate) fn parse_ambiguous_tz_annotation < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < Option < TimeZoneAnnotation < 'a , T > > > { let mut current_peek = 1 ; let critical = cursor . peek_n (current_peek) ? . map (is_critical_flag) . ok_or (ParseError :: abrupt_end ("AmbiguousAnnotation")) ? ; if critical { current_peek += 1 ; } let leading_char = cursor . peek_n (current_peek) ? . ok_or (ParseError :: abrupt_end ("AmbiguousAnnotation")) ? ; if is_a_key_leading_char (leading_char) { let mut peek_pos = current_peek + 1 ; while let Some (ch) = cursor . peek_n (peek_pos) ? { if is_annotation_key_value_separator (ch) { return Ok (None) ; } else if is_annotation_close (ch) { let tz = parse_tz_annotation (cursor) ? ; return Ok (Some (tz)) ; } peek_pos += 1 ; } Err (ParseError :: abrupt_end ("AmbiguousAnnotation")) } else { let tz = parse_tz_annotation (cursor) ? ; Ok (Some (tz)) } }
};
}
