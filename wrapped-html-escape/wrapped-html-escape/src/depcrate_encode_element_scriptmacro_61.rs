// Generated macro for macro_61 (macro)
macro_rules! Depcrate_encode_element_scriptmacro_61 {
() => {
// Module: crate::encode::element::script
// Provides: {"macro_61"}
// Dependencies: {}
encode_impl ! { 7 ; # [doc = " The following substring and characters are escaped:"] # [doc = ""] # [doc = " * `</script>` => `<\\/script>`"] # [doc = " * `\"` => `\\\"`"] # [doc = " * `'` => `\\'`"] # [doc = " * `<!--` => `<\\!--`"] parse_script_comment_quoted_text ; # [doc = " Encode text used in a quoted text in the `<script>` element."] encode_script_quoted_text ; # [doc = " Write text used in a quoted text in the `<script>` element to a mutable `String` reference and return the encoded string slice."] encode_script_quoted_text_to_string ; # [doc = " Write text used in a quoted text in the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_script_quoted_text_to_vec ; # [doc = " Write text used in a quoted text in the `<script>` element to a writer."] encode_script_quoted_text_to_writer ; }
};
}
