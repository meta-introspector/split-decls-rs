// Generated macro for macro_16 (macro)
macro_rules! Depcrate_decode_element_scriptmacro_16 {
() => {
// Module: crate::decode::element::script
// Provides: {"macro_16"}
// Dependencies: {}
decode_impl ! { 7 ; # [doc = " The following substring and character are unescaped:"] # [doc = ""] # [doc = " * `<\\/script>` => `</script>`"] # [doc = " * `\\\"` => `\"`"] parse_script_double_quoted_text ; # [doc = " Decode text from a double quoted text in the `<script>` element."] decode_script_double_quoted_text ; # [doc = " Write text from a double quoted text in the `<script>` element to a mutable `String` reference and return the encoded string slice."] decode_script_double_quoted_text_to_string ; # [doc = " Write text from a double quoted text in the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] decode_script_double_quoted_text_to_vec ; # [doc = " Write text from a double quoted text in the `<script>` element to a writer."] decode_script_double_quoted_text_to_writer ; }
};
}
