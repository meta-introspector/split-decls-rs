// Generated macro for parse_annotations (function)
macro_rules! Depcrate_parsers_annotationsparse_annotations {
() => {
// Module: crate::parsers::annotations
// Provides: {"parse_annotations"}
// Dependencies: {}
# [doc = " Parse any number of `KeyValueAnnotation`s"] pub (crate) fn parse_annotations < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > , mut handler : impl FnMut (Annotation < 'a , T >) -> Option < Annotation < 'a , T > > ,) -> ParserResult < Option < & 'a [T :: CodeUnit] > > { let mut calendar : Option < Annotation < 'a , T > > = None ; while cursor . check_or (false , is_annotation_open) ? { let annotation = handler (parse_kv_annotation (cursor) ?) ; match annotation { Some (kv) if T :: check_calendar_key (kv . key) => { match calendar { Some (calendar) if calendar . value != kv . value && (calendar . critical || kv . critical) => { return Err (ParseError :: CriticalDuplicateCalendar) } None => { calendar = Some (kv) ; } _ => { } } } Some (unknown_kv) => { if unknown_kv . critical { return Err (ParseError :: UnrecognizedCritical) ; } } None => { } } } Ok (calendar . map (| a | a . value)) }
};
}
