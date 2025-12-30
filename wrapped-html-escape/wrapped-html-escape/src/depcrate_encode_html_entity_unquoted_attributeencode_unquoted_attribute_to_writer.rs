// Generated macro for encode_unquoted_attribute_to_writer (function)
macro_rules! Depcrate_encode_html_entity_unquoted_attributeencode_unquoted_attribute_to_writer {
() => {
// Module: crate::encode::html_entity::unquoted_attribute
// Provides: {"encode_unquoted_attribute_to_writer"}
// Dependencies: {}
# [cfg (feature = "std")] # [doc = " Write text used in an unquoted attribute to a writer. Except for alphanumeric characters, escape all characters which are less than 128."] # [doc = ""] # [doc = " The following characters are escaped to named entities:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `\"` => `&quot;`"] # [doc = ""] # [doc = " Other non-alphanumeric characters are escaped to `&#xHH;`."] pub fn encode_unquoted_attribute_to_writer < S : AsRef < str > , W : Write > (text : S , output : & mut W ,) -> Result < () , io :: Error > { let text = text . as_ref () ; let text_bytes = text . as_bytes () ; let text_length = text_bytes . len () ; let mut p = 0 ; let mut e ; let mut start = 0 ; while p < text_length { e = text_bytes [p] ; if utf8_width :: is_width_1 (e) && ! e . is_ascii_alphanumeric () { output . write_all (& text_bytes [start .. p]) ? ; start = p + 1 ; write_html_entity_to_writer (e , output) ? ; } p += 1 ; } output . write_all (& text_bytes [start .. p]) }
};
}
