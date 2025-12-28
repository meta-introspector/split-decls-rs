macro_rules! inflateResetKeep {
    () => {
        # [doc (hidden)] # [doc = " ## Safety"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateResetKeep))] pub unsafe extern "C-unwind" fn inflateResetKeep (strm : * mut z_stream) -> c_int { if let Some (stream) = InflateStream :: from_stream_mut (strm) { zlib_rs :: inflate :: reset_keep (stream) as _ } else { ReturnCode :: StreamError as _ } }
    };
}

inflateResetKeep!()