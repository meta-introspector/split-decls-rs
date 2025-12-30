// Generated macro for parse_annotated_month_day (function)
macro_rules! Depcrate_parsers_datetimeparse_annotated_month_day {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_annotated_month_day"}
// Dependencies: {}
# [doc = " Parses an AnnotatedMonthDay."] pub (crate) fn parse_annotated_month_day < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > , handler : impl FnMut (Annotation < 'a , T >) -> Option < Annotation < 'a , T > > ,) -> ParserResult < IxdtfParseRecord < 'a , T > > { let date = parse_month_day (cursor) ? ; if ! cursor . check_or (false , is_annotation_open) ? { cursor . close () ? ; return Ok (IxdtfParseRecord { date : Some (date) , time : None , offset : None , tz : None , calendar : None , }) ; } let annotation_set = annotations :: parse_annotation_set (cursor , handler) ? ; Ok (IxdtfParseRecord { date : Some (date) , time : None , offset : None , tz : annotation_set . tz , calendar : annotation_set . calendar , }) }
};
}
