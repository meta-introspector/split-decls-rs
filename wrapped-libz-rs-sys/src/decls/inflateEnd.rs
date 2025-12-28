macro_rules! inflateEnd {
    () => {
        # [doc = " Deallocates all dynamically allocated data structures for this stream."] # [doc = ""] # [doc = " This function discards any unprocessed input and does not flush any pending output."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - [`Z_OK`] if success"] # [doc = " - [`Z_STREAM_ERROR`] if the stream state was inconsistent"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateEnd))] pub unsafe extern "C-unwind" fn inflateEnd (strm : * mut z_stream) -> i32 { match InflateStream :: from_stream_mut (strm) { Some (stream) => { zlib_rs :: inflate :: end (stream) ; ReturnCode :: Ok as _ } None => ReturnCode :: StreamError as _ , } }
    };
}

inflateEnd!()