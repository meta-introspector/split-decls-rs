macro_rules! other_150 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a session wants to send data to"] # [doc = " the remote peer.  This callback is not necessary if the application"] # [doc = " uses solely `nghttp2_session_mem_send()` to serialize data to"] # [doc = " transmit."] pub fn nghttp2_session_callbacks_set_send_callback (cbs : * mut nghttp2_session_callbacks , send_callback : nghttp2_send_callback ,) ; }
    };
}

other_150!();