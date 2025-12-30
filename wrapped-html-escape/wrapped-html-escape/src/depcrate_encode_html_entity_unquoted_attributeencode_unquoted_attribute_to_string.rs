// Generated macro for encode_unquoted_attribute_to_string (function)
macro_rules! Depcrate_encode_html_entity_unquoted_attributeencode_unquoted_attribute_to_string {
() => {
// Module: crate::encode::html_entity::unquoted_attribute
// Provides: {"encode_unquoted_attribute_to_string"}
// Dependencies: {}
# [doc = " Write text used in an unquoted attribute to a mutable `String` reference and return the encoded string slice. Except for alphanumeric characters, escape all characters which are less than 128."] # [doc = ""] # [doc = " The following characters are escaped to named entities:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `\"` => `&quot;`"] # [doc = ""] # [doc = " Other non-alphanumeric characters are escaped to `&#xHH;`."] # [inline] pub fn encode_unquoted_attribute_to_string < S : AsRef < str > > (text : S , output : & mut String) -> & str { unsafe { from_utf8_unchecked (encode_unquoted_attribute_to_vec (text , output . as_mut_vec ())) } }
};
}
