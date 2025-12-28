macro_rules! deps {
    () => {
        Write!();
        RangeEncoder!();
        LzmaEncoder!();
        LzmaEncoderModes!();
    };
}

macro_rules! LzmaWriter {
    () => {
        deps!();
        # [doc = " A single-threaded LZMA compressor."] pub struct LzmaWriter < W : Write > { rc : RangeEncoder < W > , lzma : LzmaEncoder , use_end_marker : bool , current_uncompressed_size : u64 , expected_uncompressed_size : Option < u64 > , props : u8 , mode : LzmaEncoderModes , }
    };
}

LzmaWriter!();