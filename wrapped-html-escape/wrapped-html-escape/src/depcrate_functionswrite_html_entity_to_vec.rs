// Generated macro for write_html_entity_to_vec (function)
macro_rules! Depcrate_functionswrite_html_entity_to_vec {
() => {
// Module: crate::functions
// Provides: {"write_html_entity_to_vec"}
// Dependencies: {}
# [inline] pub (crate) fn write_html_entity_to_vec (e : u8 , output : & mut Vec < u8 >) { match e { b'&' => output . extend_from_slice (b"&amp;") , b'<' => output . extend_from_slice (b"&lt;") , b'>' => output . extend_from_slice (b"&gt;") , b'"' => output . extend_from_slice (b"&quot;") , _ => write_hex_to_vec (e , output) , } }
};
}
