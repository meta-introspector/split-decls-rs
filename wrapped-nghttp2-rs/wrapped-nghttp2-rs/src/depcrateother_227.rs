// Generated macro for other_227 (other)
macro_rules! Depcrateother_227 {
() => {
// Module: crate
// Provides: {"other_227"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the value of SETTINGS |id| of local endpoint acknowledged"] # [doc = " by the remote endpoint.  The |id| must be one of the values defined"] # [doc = " in :enum:`nghttp2_settings_id`."] pub fn nghttp2_session_get_local_settings (session : * mut nghttp2_session , id : nghttp2_settings_id ,) -> u32 ; }
};
}
