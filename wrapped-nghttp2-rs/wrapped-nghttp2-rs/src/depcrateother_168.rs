// Generated macro for other_168 (other)
macro_rules! Depcrateother_168 {
() => {
// Module: crate
// Provides: {"other_168"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when"] # [doc = " :enum:`NGHTTP2_DATA_FLAG_NO_COPY` is used in"] # [doc = " :type:`nghttp2_data_source_read_callback` to avoid data copy."] pub fn nghttp2_session_callbacks_set_send_data_callback (cbs : * mut nghttp2_session_callbacks , send_data_callback : nghttp2_send_data_callback ,) ; }
};
}
