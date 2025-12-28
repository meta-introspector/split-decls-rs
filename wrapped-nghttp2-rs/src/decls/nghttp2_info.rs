macro_rules! nghttp2_info {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " This struct is what `nghttp2_version()` returns.  It holds"] # [doc = " information about the particular nghttp2 version."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_info { # [doc = " Age of this struct.  This instance of nghttp2 sets it to"] # [doc = " :macro:`NGHTTP2_VERSION_AGE` but a future version may bump it and"] # [doc = " add more struct fields at the bottom"] pub age : :: std :: os :: raw :: c_int , # [doc = " the :macro:`NGHTTP2_VERSION_NUM` number (since age ==1)"] pub version_num : :: std :: os :: raw :: c_int , # [doc = " points to the :macro:`NGHTTP2_VERSION` string (since age ==1)"] pub version_str : * const :: std :: os :: raw :: c_char , # [doc = " points to the :macro:`NGHTTP2_PROTO_VERSION_ID` string this"] # [doc = " instance implements (since age ==1)"] pub proto_str : * const :: std :: os :: raw :: c_char , }
    };
}

nghttp2_info!()