// Generated macro for macro_27 (macro)
macro_rules! Depcrate_decode_element_stylemacro_27 {
() => {
// Module: crate::decode::element::style
// Provides: {"macro_27"}
// Dependencies: {}
decode_impl ! { 6 ; # [doc = " The following substring and character is unescaped:"] # [doc = ""] # [doc = " * `<\\/style>` => `</style>`"] # [doc = " * `\\'` => `'`"] parse_style_single_quoted_text ; # [doc = " Decode text from a single quoted text in the `<style>` element."] decode_style_single_quoted_text ; # [doc = " Write text from a single quoted text in the `<style>` element to a mutable `String` reference and return the encoded string slice."] decode_style_single_quoted_text_to_string ; # [doc = " Write text from a single quoted text in the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] decode_style_single_quoted_text_to_vec ; # [doc = " Write text from a single quoted text in the `<style>` element to a writer."] decode_style_single_quoted_text_to_writer ; }
};
}
