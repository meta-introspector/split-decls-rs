// Generated macro for macro_101 (macro)
macro_rules! Depcrate_encode_html_entitymacro_101 {
() => {
// Module: crate::encode::html_entity
// Provides: {"macro_101"}
// Dependencies: {}
encode_impl ! { # [doc = " The following characters are escaped:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `'` => `&#x27;`"] escape_single_quote ; # [doc = " Encode text used in a single-quoted attribute."] encode_single_quoted_attribute ; # [doc = " Write text used in a single-quoted attribute to a mutable `String` reference and return the encoded string slice."] encode_single_quoted_attribute_to_string ; # [doc = " Write text used in a single-quoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_single_quoted_attribute_to_vec ; # [doc = " Write text used in a single-quoted attribute to a writer."] encode_single_quoted_attribute_to_writer ; }
};
}
