// Generated macro for is_annotation_value_component (function)
macro_rules! Depcrate_parsers_grammaris_annotation_value_component {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_annotation_value_component"}
// Dependencies: {}
# [doc = " Checks if ascii char is an `AnnotationValueComponent`."] pub (crate) const fn is_annotation_value_component (ch : u8) -> bool { ch . is_ascii_digit () || ch . is_ascii_alphabetic () }
};
}
