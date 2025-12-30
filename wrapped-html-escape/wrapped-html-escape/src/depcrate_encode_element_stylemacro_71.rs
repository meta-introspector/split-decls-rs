// Generated macro for macro_71 (macro)
macro_rules! Depcrate_encode_element_stylemacro_71 {
() => {
// Module: crate::encode::element::style
// Provides: {"macro_71"}
// Dependencies: {}
encode_impl ! { 6 ; # [doc = " The following substring and character are escaped:"] # [doc = ""] # [doc = " * `</style>` => `<\\/style>`"] # [doc = " * `'` => `\\'`"] # [doc = " * `<!--` => `<\\!--`"] parse_style_comment_single_quoted_text ; # [doc = " Encode text used in a single quoted text in the `<style>` element."] encode_style_single_quoted_text ; # [doc = " Write text used in a single quoted text in the `<style>` element to a mutable `String` reference and return the encoded string slice."] encode_style_single_quoted_text_to_string ; # [doc = " Write text used in a single quoted text in the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_style_single_quoted_text_to_vec ; # [doc = " Write text used in a single quoted text in the `<style>` element to a writer."] encode_style_single_quoted_text_to_writer ; }
};
}
