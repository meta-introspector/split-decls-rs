macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! validate_zlib_header {
    () => {
        deps!();
        # [doc = " Check that the zlib header is correct and that there is enough space in the buffer"] # [doc = " for the window size specified in the header."] # [doc = ""] # [doc = " See https://tools.ietf.org/html/rfc1950"] # [inline] const fn validate_zlib_header (cmf : u32 , flg : u32 , flags : u32 , mask : usize) -> Action { let mut failed = (((cmf * 256) + flg) % 31 != 0) || ((flg & 0b0010_0000) != 0) || ((cmf & 15) != 8) ; let window_size = 1 << ((cmf >> 4) + 8) ; if (flags & TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF) == 0 { failed |= (mask + 1) < window_size ; } failed |= window_size > 32_768 ; if failed { Action :: Jump (BadZlibHeader) } else { Action :: Jump (ReadBlockHeader) } }
    };
}

validate_zlib_header!();