macro_rules! other_225 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the value of SETTINGS |id| notified by a remote endpoint."] # [doc = " The |id| must be one of values defined in"] # [doc = " :enum:`nghttp2_settings_id`."] pub fn nghttp2_session_get_remote_settings (session : * mut nghttp2_session , id : nghttp2_settings_id ,) -> u32 ; }
    };
}

other_225!()