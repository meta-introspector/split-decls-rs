// Generated macro for escape_char (function)
macro_rules! Depcrate_escapeescape_char {
() => {
// Module: crate::escape
// Provides: {"escape_char"}
// Dependencies: {}
pub (crate) fn escape_char < W > (writer : & mut W , value : & str , from : usize , to : usize) -> fmt :: Result where W : fmt :: Write , { writer . write_str (& value [from .. to]) ? ; match value . as_bytes () [to] { b'<' => writer . write_str ("&lt;") ? , b'>' => writer . write_str ("&gt;") ? , b'\'' => writer . write_str ("&apos;") ? , b'&' => writer . write_str ("&amp;") ? , b'"' => writer . write_str ("&quot;") ? , b'\t' => writer . write_str ("&#9;") ? , b'\n' => writer . write_str ("&#10;") ? , b'\r' => writer . write_str ("&#13;") ? , b' ' => writer . write_str ("&#32;") ? , _ => unreachable ! ("Only '<', '>','\', '&', '\"', '\\t', '\\r', '\\n', and ' ' are escaped") , } Ok (()) }
};
}
