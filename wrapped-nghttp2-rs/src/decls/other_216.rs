macro_rules! other_216 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the remote window size for a given stream |stream_id|."] # [doc = ""] # [doc = " This is the amount of flow-controlled payload (e.g., DATA) that the"] # [doc = " local endpoint can send without stream level WINDOW_UPDATE.  There"] # [doc = " is also connection level flow control, so the effective size of"] # [doc = " payload that the local endpoint can actually send is"] # [doc = " min(`nghttp2_session_get_stream_remote_window_size()`,"] # [doc = " `nghttp2_session_get_remote_window_size()`)."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_stream_remote_window_size (session : * mut nghttp2_session , stream_id : i32 ,) -> i32 ; }
    };
}

other_216!()