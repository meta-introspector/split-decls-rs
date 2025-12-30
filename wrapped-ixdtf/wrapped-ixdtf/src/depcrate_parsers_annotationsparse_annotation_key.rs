// Generated macro for parse_annotation_key (function)
macro_rules! Depcrate_parsers_annotationsparse_annotation_key {
() => {
// Module: crate::parsers::annotations
// Provides: {"parse_annotation_key"}
// Dependencies: {}
# [doc = " Parse an `AnnotationKey`."] fn parse_annotation_key < 'a , T : EncodingType > (cursor : & mut Cursor < 'a , T > ,) -> ParserResult < & 'a [T :: CodeUnit] > { let key_start = cursor . pos () ; assert_syntax ! (is_a_key_leading_char (cursor . next_or (ParseError :: AnnotationKeyLeadingChar) ?) , AnnotationKeyLeadingChar ,) ; while let Some (potential_key_char) = cursor . next () ? { if cursor . check_or (false , is_annotation_key_value_separator) ? { return cursor . slice (key_start , cursor . pos ()) . ok_or (ParseError :: ImplAssert) ; } assert_syntax ! (is_a_key_char (potential_key_char) , AnnotationKeyChar) ; } Err (ParseError :: AnnotationChar) }
};
}
