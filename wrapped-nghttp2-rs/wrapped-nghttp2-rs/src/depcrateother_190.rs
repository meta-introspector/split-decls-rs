// Generated macro for other_190 (other)
macro_rules! Depcrateother_190 {
() => {
// Module: crate
// Provides: {"other_190"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " This option sets the maximum length of header block (a set of"] # [doc = " header fields per one HEADERS frame) to send.  The length of a"] # [doc = " given set of header fields is calculated using"] # [doc = " `nghttp2_hd_deflate_bound()`.  The default value is 64KiB.  If"] # [doc = " application attempts to send header fields larger than this limit,"] # [doc = " the transmission of the frame fails with error code"] # [doc = " :enum:`NGHTTP2_ERR_FRAME_SIZE_ERROR`."] pub fn nghttp2_option_set_max_send_header_block_length (option : * mut nghttp2_option , val : usize) ; }
};
}
