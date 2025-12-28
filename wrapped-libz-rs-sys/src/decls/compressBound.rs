macro_rules! compressBound {
    () => {
        # [doc = " Returns an upper bound on the compressed size after [`compress`] or [`compress2`] on `sourceLen` bytes."] # [doc = ""] # [doc = " Can be used before a [`compress`] or [`compress2`] call to allocate the destination buffer."] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (compressBound))] pub extern "C-unwind" fn compressBound (sourceLen : c_ulong) -> c_ulong { zlib_rs :: deflate :: compress_bound (sourceLen as usize) as c_ulong }
    };
}

compressBound!()