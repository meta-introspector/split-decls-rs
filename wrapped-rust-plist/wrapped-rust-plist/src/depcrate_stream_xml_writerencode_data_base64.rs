// Generated macro for encode_data_base64 (function)
macro_rules! Depcrate_stream_xml_writerencode_data_base64 {
() => {
// Module: crate::stream::xml_writer
// Provides: {"encode_data_base64"}
// Dependencies: {}
# [cfg (feature = "serde")] pub (crate) fn encode_data_base64 (data : & [u8]) -> String { let num_lines = (data . len () + DATA_MAX_LINE_BYTES - 1) / DATA_MAX_LINE_BYTES ; let max_len = num_lines * (DATA_MAX_LINE_CHARS + 1) ; let mut base64 = Vec :: with_capacity (max_len) ; write_data_base64 (data , false , b'\t' , 0 , & mut base64) . expect ("writing to a vec cannot fail") ; String :: from_utf8 (base64) . expect ("encoded base64 is ascii") }
};
}
