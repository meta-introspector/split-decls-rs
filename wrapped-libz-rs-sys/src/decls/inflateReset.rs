macro_rules! inflateReset {
    () => {
        # [doc = " Equivalent to [`inflateEnd`] followed by [`inflateInit_`], but does not free and reallocate the internal decompression state."] # [doc = ""] # [doc = " The stream will keep attributes that may have been set by [`inflateInit2_`]."] # [doc = " The stream's `total_in`, `total_out`, `adler`, and `msg` fields are initialized."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - [`Z_OK`] if success"] # [doc = " - [`Z_STREAM_ERROR`] if the source stream state was inconsistent"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateReset))] pub unsafe extern "C-unwind" fn inflateReset (strm : * mut z_stream) -> i32 { if let Some (stream) = InflateStream :: from_stream_mut (strm) { zlib_rs :: inflate :: reset (stream) as _ } else { ReturnCode :: StreamError as _ } }
    };
}

inflateReset!();