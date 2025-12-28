macro_rules! other_182 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " This option sets the SETTINGS_MAX_CONCURRENT_STREAMS value of"] # [doc = " remote endpoint as if it is received in SETTINGS frame.  Without"] # [doc = " specifying this option, the maximum number of outgoing concurrent"] # [doc = " streams is initially limited to 100 to avoid issues when the local"] # [doc = " endpoint submits lots of requests before receiving initial SETTINGS"] # [doc = " frame from the remote endpoint, since sending them at once to the"] # [doc = " remote endpoint could lead to rejection of some of the requests."] # [doc = " This value will be overwritten when the local endpoint receives"] # [doc = " initial SETTINGS frame from the remote endpoint, either to the"] # [doc = " value advertised in SETTINGS_MAX_CONCURRENT_STREAMS or to the"] # [doc = " default value (unlimited) if none was advertised."] pub fn nghttp2_option_set_peer_max_concurrent_streams (option : * mut nghttp2_option , val : u32) ; }
    };
}

other_182!()