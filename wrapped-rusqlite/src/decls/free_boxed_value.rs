macro_rules! free_boxed_value {
    () => {
        # [cfg (any (feature = "collation" , feature = "functions" , feature = "vtab"))] pub (crate) unsafe extern "C" fn free_boxed_value < T > (p : * mut std :: ffi :: c_void) { drop (Box :: from_raw (p . cast :: < T > ())) ; }
    };
}

free_boxed_value!();