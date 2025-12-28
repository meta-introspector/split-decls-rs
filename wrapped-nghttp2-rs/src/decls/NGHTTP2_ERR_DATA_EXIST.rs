macro_rules! NGHTTP2_ERR_DATA_EXIST {
    () => {
        # [doc = " DATA or HEADERS frame for a given stream has been already"] # [doc = " submitted and has not been fully processed yet.  Application"] # [doc = " should wait for the transmission of the previously submitted"] # [doc = " frame before submitting another."] pub const NGHTTP2_ERR_DATA_EXIST : nghttp2_error = - 529 ;
    };
}

NGHTTP2_ERR_DATA_EXIST!();