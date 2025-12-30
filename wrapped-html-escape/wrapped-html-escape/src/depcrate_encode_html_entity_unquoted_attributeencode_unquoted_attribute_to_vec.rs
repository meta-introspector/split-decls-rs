// Generated macro for encode_unquoted_attribute_to_vec (function)
macro_rules! Depcrate_encode_html_entity_unquoted_attributeencode_unquoted_attribute_to_vec {
() => {
// Module: crate::encode::html_entity::unquoted_attribute
// Provides: {"encode_unquoted_attribute_to_vec"}
// Dependencies: {}
# [doc = " Write text used in an unquoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice. Except for alphanumeric characters, escape all characters which are less than 128."] # [doc = ""] # [doc = " The following characters are escaped to named entities:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `\"` => `&quot;`"] # [doc = ""] # [doc = " Other non-alphanumeric characters are escaped to `&#xHH;`."] pub fn encode_unquoted_attribute_to_vec < S : AsRef < str > > (text : S , output : & mut Vec < u8 >) -> & [u8] { let text = text . as_ref () ; let text_bytes = text . as_bytes () ; let text_length = text_bytes . len () ; output . reserve (text_length) ; let current_length = output . len () ; let mut p = 0 ; let mut e ; let mut start = 0 ; while p < text_length { e = text_bytes [p] ; if utf8_width :: is_width_1 (e) && ! e . is_ascii_alphanumeric () { output . extend_from_slice (& text_bytes [start .. p]) ; start = p + 1 ; write_html_entity_to_vec (e , output) ; } p += 1 ; } output . extend_from_slice (& text_bytes [start .. p]) ; & output [current_length ..] }
};
}
