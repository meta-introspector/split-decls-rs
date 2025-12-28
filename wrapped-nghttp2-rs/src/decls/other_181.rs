macro_rules! other_181 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " This option prevents the library from sending WINDOW_UPDATE for a"] # [doc = " connection automatically.  If this option is set to nonzero, the"] # [doc = " library won't send WINDOW_UPDATE for DATA until application calls"] # [doc = " `nghttp2_session_consume()` to indicate the consumed amount of"] # [doc = " data.  Don't use `nghttp2_submit_window_update()` for this purpose."] # [doc = " By default, this option is set to zero."] pub fn nghttp2_option_set_no_auto_window_update (option : * mut nghttp2_option , val : :: std :: os :: raw :: c_int ,) ; }
    };
}

other_181!();