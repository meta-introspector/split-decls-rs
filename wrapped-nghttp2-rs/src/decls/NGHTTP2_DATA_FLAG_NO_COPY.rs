macro_rules! NGHTTP2_DATA_FLAG_NO_COPY {
    () => {
        # [doc = " Indicates that application will send complete DATA frame in"] # [doc = " :type:`nghttp2_send_data_callback`."] pub const NGHTTP2_DATA_FLAG_NO_COPY : nghttp2_data_flag = 4 ;
    };
}

NGHTTP2_DATA_FLAG_NO_COPY!();