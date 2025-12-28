macro_rules! other_98 {
    () => {
        # [doc = " @union"] # [doc = ""] # [doc = " This union represents the some kind of data source passed to"] # [doc = " :type:`nghttp2_data_source_read_callback`."] # [repr (C)] # [derive (Copy , Clone)] pub union nghttp2_data_source { # [doc = " The integer field, suitable for a file descriptor."] pub fd : :: std :: os :: raw :: c_int , # [doc = " The pointer to an arbitrary object."] pub ptr : * mut :: std :: os :: raw :: c_void , _bindgen_union_align : u64 , }
    };
}

other_98!();