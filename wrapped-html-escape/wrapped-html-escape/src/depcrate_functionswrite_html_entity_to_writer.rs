// Generated macro for write_html_entity_to_writer (function)
macro_rules! Depcrate_functionswrite_html_entity_to_writer {
() => {
// Module: crate::functions
// Provides: {"write_html_entity_to_writer"}
// Dependencies: {}
# [cfg (feature = "std")] # [inline] pub (crate) fn write_html_entity_to_writer < W : Write > (e : u8 , output : & mut W ,) -> Result < () , io :: Error > { match e { b'&' => output . write_all (b"&amp;") , b'<' => output . write_all (b"&lt;") , b'>' => output . write_all (b"&gt;") , b'"' => output . write_all (b"&quot;") , _ => write_hex_to_writer (e , output) , } }
};
}
