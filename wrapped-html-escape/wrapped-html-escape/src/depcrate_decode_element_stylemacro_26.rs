// Generated macro for macro_26 (macro)
macro_rules! Depcrate_decode_element_stylemacro_26 {
() => {
// Module: crate::decode::element::style
// Provides: {"macro_26"}
// Dependencies: {}
decode_impl ! { 6 ; # [doc = " The following substring is unescaped:"] # [doc = ""] # [doc = " * `<\\/style>` => `</style>`"] parse_style ; # [doc = " Decode text from the `<style>` element."] decode_style ; # [doc = " Write text from the `<style>` element to a mutable `String` reference and return the encoded string slice."] decode_style_to_string ; # [doc = " Write text from the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] decode_style_to_vec ; # [doc = " Write text from the `<style>` element to a writer."] decode_style_to_writer ; }
};
}
