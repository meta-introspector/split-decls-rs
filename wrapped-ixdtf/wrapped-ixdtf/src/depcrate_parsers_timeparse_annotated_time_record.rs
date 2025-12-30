// Generated macro for parse_annotated_time_record (function)
macro_rules! Depcrate_parsers_timeparse_annotated_time_record {
() => {
// Module: crate::parsers::time
// Provides: {"parse_annotated_time_record"}
// Dependencies: {}
# [doc = " Parse annotated time record is silently fallible returning None in the case that the"] # [doc = " value does not align"] pub (crate) fn parse_annotated_time_record < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > , handler : impl FnMut (Annotation < 'a , T >) -> Option < Annotation < 'a , T > > ,) -> ParserResult < IxdtfParseRecord < 'a , T > > { let start = cursor . pos () ; let designator = cursor . check_or (false , is_time_designator) ? ; cursor . advance_if (designator) ; let time = parse_time_record (cursor) ? ; let offset = if cursor . check_or (false , | ch | is_ascii_sign (ch) || is_utc_designator (ch)) ? { Some (timezone :: parse_date_time_utc_offset (cursor) ?) } else { None } ; if ! cursor . check_or (false , is_annotation_open) ? { cursor . close () ? ; check_time_ambiguity (cursor , start) ? ; return Ok (IxdtfParseRecord { date : None , time : Some (time) , offset , tz : None , calendar : None , }) ; } check_time_ambiguity (cursor , start) ? ; let annotations = annotations :: parse_annotation_set (cursor , handler) ? ; cursor . close () ? ; Ok (IxdtfParseRecord { date : None , time : Some (time) , offset , tz : annotations . tz , calendar : annotations . calendar , }) }
};
}
