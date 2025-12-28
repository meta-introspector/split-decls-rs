macro_rules! nghttp2_window_update {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The WINDOW_UPDATE frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_window_update { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The window size increment."] pub window_size_increment : i32 , # [doc = " Reserved bit.  Currently this is always set to 0 and application"] # [doc = " should not expect something useful in here."] pub reserved : u8 , }
    };
}

nghttp2_window_update!()