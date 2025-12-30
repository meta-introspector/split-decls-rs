// Generated macro for macro_28 (macro)
macro_rules! Depcrate_decode_element_stylemacro_28 {
() => {
// Module: crate::decode::element::style
// Provides: {"macro_28"}
// Dependencies: {}
decode_impl ! { 6 ; # [doc = " The following substring and character are unescaped:"] # [doc = ""] # [doc = " * `<\\/style>` => `</style>`"] # [doc = " * `\\\"` => `\"`"] parse_style_double_quoted_text ; # [doc = " Decode text from a double quoted text in the `<style>` element."] decode_style_double_quoted_text ; # [doc = " Write text from a double quoted text in the `<style>` element to a mutable `String` reference and return the encoded string slice."] decode_style_double_quoted_text_to_string ; # [doc = " Write text from a double quoted text in the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] decode_style_double_quoted_text_to_vec ; # [doc = " Write text from a double quoted text in the `<style>` element to a writer."] decode_style_double_quoted_text_to_writer ; }
};
}
