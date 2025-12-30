// Generated macro for parse_annotation_value (function)
macro_rules! Depcrate_parsers_annotationsparse_annotation_value {
() => {
// Module: crate::parsers::annotations
// Provides: {"parse_annotation_value"}
// Dependencies: {}
# [doc = " Parse an `AnnotationValue`."] fn parse_annotation_value < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < & 'a [T :: CodeUnit] > { let value_start = cursor . pos () ; cursor . advance () ; while let Some (potential_value_char) = cursor . next () ? { if cursor . check_or (false , is_annotation_close) ? { return cursor . slice (value_start , cursor . pos ()) . ok_or (ParseError :: ImplAssert) ; } if is_hyphen (potential_value_char) { assert_syntax ! (cursor . peek () ?. is_some_and (is_annotation_value_component) , AnnotationValueCharPostHyphen ,) ; cursor . advance () ; continue ; } assert_syntax ! (is_annotation_value_component (potential_value_char) , AnnotationValueChar ,) ; } Err (ParseError :: AnnotationValueChar) }
};
}
