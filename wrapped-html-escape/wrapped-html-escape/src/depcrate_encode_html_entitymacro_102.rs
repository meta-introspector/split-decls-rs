// Generated macro for macro_102 (macro)
macro_rules! Depcrate_encode_html_entitymacro_102 {
() => {
// Module: crate::encode::html_entity
// Provides: {"macro_102"}
// Dependencies: {}
encode_impl ! { # [doc = " The following characters (HTML reserved characters)  are escaped:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `\"` => `&quot;`"] # [doc = " * `'` => `&#x27;`"] escape_quote ; # [doc = " Encode text used in a quoted attribute."] encode_quoted_attribute ; # [doc = " Write text used in a quoted attribute to a mutable `String` reference and return the encoded string slice."] encode_quoted_attribute_to_string ; # [doc = " Write text used in a quoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_quoted_attribute_to_vec ; # [doc = " Write text used in a quoted attribute to a writer."] encode_quoted_attribute_to_writer ; }
};
}
