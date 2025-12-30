// Generated macro for parse_annotation_set (function)
macro_rules! Depcrate_parsers_annotationsparse_annotation_set {
() => {
// Module: crate::parsers::annotations
// Provides: {"parse_annotation_set"}
// Dependencies: {}
# [doc = " Parse a `TimeZoneAnnotation` `Annotations` set"] pub (crate) fn parse_annotation_set < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > , handler : impl FnMut (Annotation < 'a , T >) -> Option < Annotation < 'a , T > > ,) -> ParserResult < AnnotationSet < 'a , T > > { let tz_annotation = timezone :: parse_ambiguous_tz_annotation (cursor) ? ; let annotations = cursor . check_or (false , is_annotation_open) ? ; if annotations { let calendar = parse_annotations (cursor , handler) ? ; return Ok (AnnotationSet { tz : tz_annotation , calendar , }) ; } Ok (AnnotationSet { tz : tz_annotation , calendar : None , }) }
};
}
