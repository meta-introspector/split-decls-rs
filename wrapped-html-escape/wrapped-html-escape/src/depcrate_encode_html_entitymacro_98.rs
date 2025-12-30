// Generated macro for macro_98 (macro)
macro_rules! Depcrate_encode_html_entitymacro_98 {
() => {
// Module: crate::encode::html_entity
// Provides: {"macro_98"}
// Dependencies: {}
encode_impl ! { # [doc = " The following characters are escaped:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] escape_text_minimal ; # [doc = " Encode text used as regular HTML text."] encode_text_minimal ; # [doc = " Write text used as regular HTML text to a mutable `String` reference and return the encoded string slice."] encode_text_minimal_to_string ; # [doc = " Write text used as regular HTML text to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_text_minimal_to_vec ; # [doc = " Write text used as regular HTML text to a writer."] encode_text_minimal_to_writer ; }
};
}
