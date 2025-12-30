// Generated macro for parse_kv_annotation (function)
macro_rules! Depcrate_parsers_annotationsparse_kv_annotation {
() => {
// Module: crate::parsers::annotations
// Provides: {"parse_kv_annotation"}
// Dependencies: {}
# [doc = " Parse an annotation with an `AnnotationKey`=`AnnotationValue` pair."] fn parse_kv_annotation < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < Annotation < 'a , T > > { assert_syntax ! (is_annotation_open (cursor . next_or (ParseError :: AnnotationOpen) ?) , AnnotationOpen) ; let critical = cursor . check_or (false , is_critical_flag) ? ; cursor . advance_if (critical) ; let annotation_key = parse_annotation_key (cursor) ? ; assert_syntax ! (is_annotation_key_value_separator (cursor . next_or (ParseError :: AnnotationKeyValueSeparator) ?) , AnnotationKeyValueSeparator ,) ; let annotation_value = parse_annotation_value (cursor) ? ; assert_syntax ! (is_annotation_close (cursor . next_or (ParseError :: AnnotationClose) ?) , AnnotationClose) ; Ok (Annotation { critical , key : annotation_key , value : annotation_value , }) }
};
}
