macro_rules! nghttp2_settings_entry {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The SETTINGS ID/Value pair.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_settings_entry { # [doc = " The SETTINGS ID.  See :type:`nghttp2_settings_id`."] pub settings_id : i32 , # [doc = " The value of this entry."] pub value : u32 , }
    };
}

nghttp2_settings_entry!()