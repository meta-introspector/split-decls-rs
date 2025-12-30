// Generated macro for other_226 (other)
macro_rules! Depcrateother_226 {
() => {
// Module: crate
// Provides: {"other_226"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the value of SETTINGS |id| notified by a remote endpoint."] # [doc = " The |id| must be one of values defined in"] # [doc = " :enum:`nghttp2_settings_id`."] pub fn nghttp2_session_get_remote_settings (session : * mut nghttp2_session , id : nghttp2_settings_id ,) -> u32 ; }
};
}
