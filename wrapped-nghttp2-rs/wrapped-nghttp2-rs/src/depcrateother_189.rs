// Generated macro for other_189 (other)
macro_rules! Depcrateother_189 {
() => {
// Module: crate
// Provides: {"other_189"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " This option prevents the library from sending PING frame with ACK"] # [doc = " flag set automatically when PING frame without ACK flag set is"] # [doc = " received.  If this option is set to nonzero, the library won't send"] # [doc = " PING frame with ACK flag set in the response for incoming PING"] # [doc = " frame.  The application can send PING frame with ACK flag set using"] # [doc = " `nghttp2_submit_ping()` with :enum:`NGHTTP2_FLAG_ACK` as flags"] # [doc = " parameter."] pub fn nghttp2_option_set_no_auto_ping_ack (option : * mut nghttp2_option , val : :: std :: os :: raw :: c_int ,) ; }
};
}
