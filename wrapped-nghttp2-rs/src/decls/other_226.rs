macro_rules! other_226 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the value of SETTINGS |id| of local endpoint acknowledged"] # [doc = " by the remote endpoint.  The |id| must be one of the values defined"] # [doc = " in :enum:`nghttp2_settings_id`."] pub fn nghttp2_session_get_local_settings (session : * mut nghttp2_session , id : nghttp2_settings_id ,) -> u32 ; }
    };
}

other_226!()