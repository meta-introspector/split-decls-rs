// Generated macro for nghttp2_data (struct)
macro_rules! Depcratenghttp2_data {
() => {
// Module: crate
// Provides: {"nghttp2_data"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The DATA frame.  The received data is delivered via"] # [doc = " :type:`nghttp2_on_data_chunk_recv_callback`."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_data { pub hd : nghttp2_frame_hd , # [doc = " The length of the padding in this frame.  This includes PAD_HIGH"] # [doc = " and PAD_LOW."] pub padlen : usize , }
};
}
