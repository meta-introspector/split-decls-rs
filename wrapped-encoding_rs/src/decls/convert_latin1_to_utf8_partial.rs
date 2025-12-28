macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! convert_latin1_to_utf8_partial {
    () => {
        deps!();
        # [doc = " Converts bytes whose unsigned value is interpreted as Unicode code point"] # [doc = " (i.e. U+0000 to U+00FF, inclusive) to UTF-8 with potentially insufficient"] # [doc = " output space."] # [doc = ""] # [doc = " Returns the number of bytes read and the number of bytes written."] # [doc = ""] # [doc = " If the output isn't large enough, not all input is consumed."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If you want to convert into a `&mut str`, use"] # [doc = " `convert_utf16_to_str_partial()` instead of using this function"] # [doc = " together with the `unsafe` method `as_bytes_mut()` on `&mut str`."] pub fn convert_latin1_to_utf8_partial (src : & [u8] , dst : & mut [u8]) -> (usize , usize) { let src_len = src . len () ; let src_ptr = src . as_ptr () ; let dst_ptr = dst . as_mut_ptr () ; let dst_len = dst . len () ; let mut total_read = 0usize ; let mut total_written = 0usize ; loop { let src_left = src_len - total_read ; let dst_left = dst_len - total_written ; let min_left = :: core :: cmp :: min (src_left , dst_left) ; if let Some ((non_ascii , consumed)) = unsafe { ascii_to_ascii (src_ptr . add (total_read) , dst_ptr . add (total_written) , min_left ,) } { total_read += consumed ; total_written += consumed ; if total_written . checked_add (2) . unwrap () > dst_len { return (total_read , total_written) ; } total_read += 1 ; dst [total_written] = (non_ascii >> 6) | 0xC0 ; total_written += 1 ; dst [total_written] = (non_ascii & 0x3F) | 0x80 ; total_written += 1 ; continue ; } return (total_read + min_left , total_written + min_left) ; } }
    };
}

convert_latin1_to_utf8_partial!()