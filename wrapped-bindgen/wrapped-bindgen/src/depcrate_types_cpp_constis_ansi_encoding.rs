// Generated macro for is_ansi_encoding (function)
macro_rules! Depcrate_types_cpp_constis_ansi_encoding {
() => {
// Module: crate::types::cpp_const
// Provides: {"is_ansi_encoding"}
// Dependencies: {}
fn is_ansi_encoding (row : Field) -> bool { row . find_attribute ("NativeEncodingAttribute") . is_some_and (| attribute | matches ! (attribute . args () . first () , Some ((_ , Value :: Str (encoding))) if * encoding == "ansi")) }
};
}
