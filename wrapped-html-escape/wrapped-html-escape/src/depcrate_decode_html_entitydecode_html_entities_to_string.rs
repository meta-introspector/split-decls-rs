// Generated macro for decode_html_entities_to_string (function)
macro_rules! Depcrate_decode_html_entitydecode_html_entities_to_string {
() => {
// Module: crate::decode::html_entity
// Provides: {"decode_html_entities_to_string"}
// Dependencies: {}
# [doc = " Decode html entities in a given string to a mutable `String` reference and return the decoded string slice."] pub fn decode_html_entities_to_string < S : AsRef < str > > (text : S , output : & mut String) -> & str { unsafe { from_utf8_unchecked (decode_html_entities_to_vec (text , output . as_mut_vec ())) } }
};
}
