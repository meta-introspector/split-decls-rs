// Generated macro for write_data_base64 (function)
macro_rules! Depcrate_stream_xml_writerwrite_data_base64 {
() => {
// Module: crate::stream::xml_writer
// Provides: {"write_data_base64"}
// Dependencies: {}
fn write_data_base64 (data : & [u8] , write_initial_newline : bool , indent_char : u8 , indent_repeat : usize , mut writer : impl Write ,) -> io :: Result < () > { let mut encoded = [0 ; DATA_MAX_LINE_CHARS] ; for (i , line) in data . chunks (DATA_MAX_LINE_BYTES) . enumerate () { if write_initial_newline || i > 0 { writer . write_all (b"\n") ? ; } for _ in 0 .. indent_repeat { writer . write_all (& [indent_char]) ? ; } let encoded_len = BASE64_STANDARD . encode_slice (line , & mut encoded) . expect ("encoded base64 max line length is known") ; writer . write_all (& encoded [.. encoded_len]) ? ; } Ok (()) }
};
}
