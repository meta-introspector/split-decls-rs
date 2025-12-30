// Generated macro for other_165 (other)
macro_rules! Depcrateother_165 {
() => {
// Module: crate
// Provides: {"other_165"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the library asks application"] # [doc = " how many padding bytes are required for the transmission of the"] # [doc = " given frame."] pub fn nghttp2_session_callbacks_set_select_padding_callback (cbs : * mut nghttp2_session_callbacks , select_padding_callback : nghttp2_select_padding_callback ,) ; }
};
}
