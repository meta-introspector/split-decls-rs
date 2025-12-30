// Generated macro for parse_tz_annotation (function)
macro_rules! Depcrate_parsers_timezoneparse_tz_annotation {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_tz_annotation"}
// Dependencies: {}
fn parse_tz_annotation < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < TimeZoneAnnotation < 'a , T > > { assert_syntax ! (is_annotation_open (cursor . next_or (ParseError :: AnnotationOpen) ?) , AnnotationOpen) ; let critical = cursor . check_or (false , is_critical_flag) ? ; cursor . advance_if (critical) ; let tz = parse_time_zone (cursor) ? ; assert_syntax ! (is_annotation_close (cursor . next_or (ParseError :: AnnotationClose) ?) , AnnotationClose) ; Ok (TimeZoneAnnotation { critical , tz }) }
};
}
