// Generated macro for encode_unquoted_attribute (function)
macro_rules! Depcrate_encode_html_entity_unquoted_attributeencode_unquoted_attribute {
() => {
// Module: crate::encode::html_entity::unquoted_attribute
// Provides: {"encode_unquoted_attribute"}
// Dependencies: {}
# [doc = " Encode text used in an unquoted attribute. Except for alphanumeric characters, escape all characters which are less than 128."] # [doc = ""] # [doc = " The following characters are escaped to named entities:"] # [doc = ""] # [doc = " * `&` => `&amp;`"] # [doc = " * `<` => `&lt;`"] # [doc = " * `>` => `&gt;`"] # [doc = " * `\"` => `&quot;`"] # [doc = ""] # [doc = " Other non-alphanumeric characters are escaped to `&#xHH;`."] pub fn encode_unquoted_attribute < S : ? Sized + AsRef < str > > (text : & S) -> Cow < str > { let text = text . as_ref () ; let text_bytes = text . as_bytes () ; let text_length = text_bytes . len () ; let mut p = 0 ; let mut e ; loop { if p == text_length { return Cow :: from (text) ; } e = text_bytes [p] ; if utf8_width :: is_width_1 (e) && ! e . is_ascii_alphanumeric () { break ; } p += 1 ; } let mut v = Vec :: with_capacity (text_length) ; v . extend_from_slice (& text_bytes [.. p]) ; write_html_entity_to_vec (e , & mut v) ; encode_unquoted_attribute_to_vec (unsafe { from_utf8_unchecked (& text_bytes [(p + 1) ..]) } , & mut v ,) ; Cow :: from (unsafe { String :: from_utf8_unchecked (v) }) }
};
}
