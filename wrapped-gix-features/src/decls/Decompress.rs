macro_rules! Decompress {
    () => {
        # [doc = " A type to hold all state needed for decompressing a ZLIB encoded stream."] pub struct Decompress (libz_rs_sys :: z_stream) ;
    };
}

Decompress!();