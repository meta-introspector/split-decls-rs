macro_rules! nghttp2_ping {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The PING frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_ping { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The opaque data"] pub opaque_data : [u8 ; 8usize] , }
    };
}

nghttp2_ping!()