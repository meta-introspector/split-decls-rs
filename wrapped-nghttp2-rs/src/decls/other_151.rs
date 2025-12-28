macro_rules! other_151 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the a session wants to receive"] # [doc = " data from the remote peer.  This callback is not necessary if the"] # [doc = " application uses solely `nghttp2_session_mem_recv()` to process"] # [doc = " received data."] pub fn nghttp2_session_callbacks_set_recv_callback (cbs : * mut nghttp2_session_callbacks , recv_callback : nghttp2_recv_callback ,) ; }
    };
}

other_151!()