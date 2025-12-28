macro_rules! inflateCodesUsed {
    () => {
        # [doc (hidden)] # [doc = " Returns the number of codes used"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that either:"] # [doc = ""] # [doc = " - `buf` is `NULL`"] # [doc = " - `buf` and `len` satisfy the requirements of [`core::slice::from_raw_parts`]"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateCodesUsed))] pub unsafe extern "C-unwind" fn inflateCodesUsed (strm : * mut z_stream) -> c_ulong { match InflateStream :: from_stream_mut (strm) { Some (stream) => zlib_rs :: inflate :: codes_used (stream) as c_ulong , None => c_ulong :: MAX , } }
    };
}

inflateCodesUsed!();