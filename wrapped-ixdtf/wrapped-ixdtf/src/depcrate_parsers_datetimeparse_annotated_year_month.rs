// Generated macro for parse_annotated_year_month (function)
macro_rules! Depcrate_parsers_datetimeparse_annotated_year_month {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_annotated_year_month"}
// Dependencies: {}
# [doc = " Parse an annotated YearMonth"] pub (crate) fn parse_annotated_year_month < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > , handler : impl FnMut (Annotation < 'a , T >) -> Option < Annotation < 'a , T > > ,) -> ParserResult < IxdtfParseRecord < 'a , T > > { let year_month = parse_year_month (cursor) ? ; if ! cursor . check_or (false , is_annotation_open) ? { cursor . close () ? ; return Ok (IxdtfParseRecord { date : Some (year_month) , time : None , offset : None , tz : None , calendar : None , }) ; } let annotation_set = annotations :: parse_annotation_set (cursor , handler) ? ; Ok (IxdtfParseRecord { date : Some (year_month) , time : None , offset : None , tz : annotation_set . tz , calendar : annotation_set . calendar , }) }
};
}
