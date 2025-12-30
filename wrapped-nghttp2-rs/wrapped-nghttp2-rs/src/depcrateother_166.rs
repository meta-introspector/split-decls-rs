// Generated macro for other_166 (other)
macro_rules! Depcrateother_166 {
() => {
// Module: crate
// Provides: {"other_166"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function determine the length allowed in"] # [doc = " :type:`nghttp2_data_source_read_callback`."] pub fn nghttp2_session_callbacks_set_data_source_read_length_callback (cbs : * mut nghttp2_session_callbacks , data_source_read_length_callback : nghttp2_data_source_read_length_callback ,) ; }
};
}
