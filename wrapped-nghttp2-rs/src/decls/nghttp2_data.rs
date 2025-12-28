macro_rules! nghttp2_data {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The DATA frame.  The received data is delivered via"] # [doc = " :type:`nghttp2_on_data_chunk_recv_callback`."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_data { pub hd : nghttp2_frame_hd , # [doc = " The length of the padding in this frame.  This includes PAD_HIGH"] # [doc = " and PAD_LOW."] pub padlen : usize , }
    };
}

nghttp2_data!();