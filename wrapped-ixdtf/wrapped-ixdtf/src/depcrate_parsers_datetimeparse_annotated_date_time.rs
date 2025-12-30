// Generated macro for parse_annotated_date_time (function)
macro_rules! Depcrate_parsers_datetimeparse_annotated_date_time {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_annotated_date_time"}
// Dependencies: {}
# [doc = " This function handles parsing for [`AnnotatedDateTime`][datetime],"] # [doc = " [`AnnotatedDateTimeTimeRequred`][time], and"] # [doc = " [`TemporalInstantString.`][instant] according to the requirements"] # [doc = " provided via Spec."] # [doc = ""] # [doc = " [datetime]: https://tc39.es/proposal-temporal/#prod-AnnotatedDateTime"] # [doc = " [time]: https://tc39.es/proposal-temporal/#prod-AnnotatedDateTimeTimeRequired"] # [doc = " [instant]: https://tc39.es/proposal-temporal/#prod-TemporalInstantString"] pub (crate) fn parse_annotated_date_time < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > , handler : impl FnMut (Annotation < 'a , T >) -> Option < Annotation < 'a , T > > ,) -> ParserResult < IxdtfParseRecord < 'a , T > > { let date_time = parse_date_time (cursor) ? ; if ! cursor . check_or (false , is_annotation_open) ? { cursor . close () ? ; return Ok (IxdtfParseRecord { date : date_time . date , time : date_time . time , offset : date_time . time_zone , tz : None , calendar : None , }) ; } let annotation_set = annotations :: parse_annotation_set (cursor , handler) ? ; cursor . close () ? ; Ok (IxdtfParseRecord { date : date_time . date , time : date_time . time , offset : date_time . time_zone , tz : annotation_set . tz , calendar : annotation_set . calendar , }) }
};
}
