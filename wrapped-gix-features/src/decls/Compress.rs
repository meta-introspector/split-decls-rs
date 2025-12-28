macro_rules! Compress {
    () => {
        # [doc = " Hold all state needed for compressing data."] pub struct Compress (libz_rs_sys :: z_stream) ;
    };
}

Compress!();