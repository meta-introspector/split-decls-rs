macro_rules! other_254 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero if new request can be sent from local endpoint."] # [doc = ""] # [doc = " This function return 0 if request is not allowed for this session."] # [doc = " There are several reasons why request is not allowed.  Some of the"] # [doc = " reasons are: session is server; stream ID has been spent; GOAWAY"] # [doc = " has been sent or received."] # [doc = ""] # [doc = " The application can call `nghttp2_submit_request()` without"] # [doc = " consulting this function.  In that case, `nghttp2_submit_request()`"] # [doc = " may return error.  Or, request is failed to sent, and"] # [doc = " :type:`nghttp2_on_stream_close_callback` is called."] pub fn nghttp2_session_check_request_allowed (session : * mut nghttp2_session ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_254!()