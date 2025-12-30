// Generated macro for macro_70 (macro)
macro_rules! Depcrate_encode_element_stylemacro_70 {
() => {
// Module: crate::encode::element::style
// Provides: {"macro_70"}
// Dependencies: {}
encode_impl ! { 6 ; # [doc = " The following substring is escaped:"] # [doc = ""] # [doc = " * `</style>` => `<\\/style>`"] # [doc = " * `<!--` => `<\\!--`"] parse_style_comment ; # [doc = " Encode text used in the `<style>` element."] encode_style ; # [doc = " Write text used in the `<style>` element to a mutable `String` reference and return the encoded string slice."] encode_style_to_string ; # [doc = " Write text used in the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_style_to_vec ; # [doc = " Write text used in the `<style>` element to a writer."] encode_style_to_writer ; }
};
}
