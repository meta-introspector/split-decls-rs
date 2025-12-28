macro_rules! other_212 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the amount of flow-controlled payload (e.g., DATA) that the"] # [doc = " remote endpoint can send without receiving stream level"] # [doc = " WINDOW_UPDATE frame.  It is also subject to the connection level"] # [doc = " flow control.  So the actual amount of data to send is"] # [doc = " min(`nghttp2_session_get_stream_local_window_size()`,"] # [doc = " `nghttp2_session_get_local_window_size()`)."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_stream_local_window_size (session : * mut nghttp2_session , stream_id : i32 ,) -> i32 ; }
    };
}

other_212!()