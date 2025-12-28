macro_rules! deps {
    () => {
        TINFLStatus!();
    };
}

macro_rules! DecompressError {
    () => {
        deps!();
        # [doc = " Struct return when decompress_to_vec functions fail."] # [cfg (feature = "with-alloc")] # [derive (Debug)] pub struct DecompressError { # [doc = " Decompressor status on failure. See [TINFLStatus] for details."] pub status : TINFLStatus , # [doc = " The currently decompressed data if any."] pub output : Vec < u8 > , }
    };
}

DecompressError!()