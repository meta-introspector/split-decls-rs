// Generated macro for macro_100 (macro)
macro_rules! Depcrate_encode_html_entitymacro_100 {
() => {
// Module: crate::encode::html_entity
// Provides: {"macro_100"}
// Dependencies: {}
encode_impl ! { # [doc = " The following characters are escaped:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `\"` => `&quot;`"] escape_double_quote ; # [doc = " Encode text used in a double-quoted attribute."] encode_double_quoted_attribute ; # [doc = " Write text used in a double-quoted attribute to a mutable `String` reference and return the encoded string slice."] encode_double_quoted_attribute_to_string ; # [doc = " Write text used in a double-quoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_double_quoted_attribute_to_vec ; # [doc = " Write text used in a double-quoted attribute to a writer."] encode_double_quoted_attribute_to_writer ; }
};
}
