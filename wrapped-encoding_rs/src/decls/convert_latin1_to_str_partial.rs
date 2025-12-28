macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! convert_latin1_to_str_partial {
    () => {
        deps!();
        # [doc = " Converts bytes whose unsigned value is interpreted as Unicode code point"] # [doc = " (i.e. U+0000 to U+00FF, inclusive) to UTF-8 such that the validity of the"] # [doc = " output is signaled using the Rust type system with potentially insufficient"] # [doc = " output space."] # [doc = ""] # [doc = " Returns the number of bytes read and the number of bytes written."] # [doc = ""] # [doc = " If the output isn't large enough, not all input is consumed."] # [inline] pub fn convert_latin1_to_str_partial (src : & [u8] , dst : & mut str) -> (usize , usize) { let bytes : & mut [u8] = unsafe { dst . as_bytes_mut () } ; let (read , written) = convert_latin1_to_utf8_partial (src , bytes) ; let len = bytes . len () ; let mut trail = written ; let max = :: core :: cmp :: min (len , trail + MAX_STRIDE_SIZE) ; while trail < max { bytes [trail] = 0 ; trail += 1 ; } while trail < len && ((bytes [trail] & 0xC0) == 0x80) { bytes [trail] = 0 ; trail += 1 ; } (read , written) }
    };
}

convert_latin1_to_str_partial!()