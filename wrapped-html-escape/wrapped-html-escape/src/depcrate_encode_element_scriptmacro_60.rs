// Generated macro for macro_60 (macro)
macro_rules! Depcrate_encode_element_scriptmacro_60 {
() => {
// Module: crate::encode::element::script
// Provides: {"macro_60"}
// Dependencies: {}
encode_impl ! { 7 ; # [doc = " The following substring and character are escaped:"] # [doc = ""] # [doc = " * `</script>` => `<\\/script>`"] # [doc = " * `\"` => `\\\"`"] # [doc = " * `<!--` => `<\\!--`"] parse_script_comment_double_quoted_text ; # [doc = " Encode text used in a double quoted text in the `<script>` element."] encode_script_double_quoted_text ; # [doc = " Write text used in a double quoted text in the `<script>` element to a mutable `String` reference and return the encoded string slice."] encode_script_double_quoted_text_to_string ; # [doc = " Write text used in a double quoted text in the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_script_double_quoted_text_to_vec ; # [doc = " Write text used in a double quoted text in the `<script>` element to a writer."] encode_script_double_quoted_text_to_writer ; }
};
}
