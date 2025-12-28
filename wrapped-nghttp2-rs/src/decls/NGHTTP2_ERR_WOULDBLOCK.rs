macro_rules! NGHTTP2_ERR_WOULDBLOCK {
    () => {
        # [doc = " Used as a return value from :type:`nghttp2_send_callback`,"] # [doc = " :type:`nghttp2_recv_callback` and"] # [doc = " :type:`nghttp2_send_data_callback` to indicate that the operation"] # [doc = " would block."] pub const NGHTTP2_ERR_WOULDBLOCK : nghttp2_error = - 504 ;
    };
}

NGHTTP2_ERR_WOULDBLOCK!();