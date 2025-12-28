macro_rules! deps {
    () => {
        DecompressError!();
        TINFLStatus!();
        Result!();
    };
}

macro_rules! decompress_error {
    () => {
        deps!();
        # [cfg (feature = "with-alloc")] fn decompress_error (status : TINFLStatus , output : Vec < u8 >) -> Result < Vec < u8 > , DecompressError > { Err (DecompressError { status , output }) }
    };
}

decompress_error!();