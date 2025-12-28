macro_rules! deps {
    () => {
        Compress!();
    };
}

macro_rules! Write {
    () => {
        deps!();
        # [doc = " A utility to zlib compress anything that is written via its [Write][std::io::Write] implementation."] # [doc = ""] # [doc = " Be sure to call `flush()` when done to finalize the deflate stream."] pub struct Write < W > { compressor : Compress , inner : W , buf : [u8 ; BUF_SIZE] , }
    };
}

Write!()