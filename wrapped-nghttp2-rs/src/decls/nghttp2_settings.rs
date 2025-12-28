macro_rules! nghttp2_settings {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The SETTINGS frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_settings { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The number of SETTINGS ID/Value pairs in |iv|."] pub niv : usize , # [doc = " The pointer to the array of SETTINGS ID/Value pair."] pub iv : * mut nghttp2_settings_entry , }
    };
}

nghttp2_settings!();