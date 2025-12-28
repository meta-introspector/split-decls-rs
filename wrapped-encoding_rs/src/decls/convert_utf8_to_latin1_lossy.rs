macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! convert_utf8_to_latin1_lossy {
    () => {
        deps!();
        # [doc = " If the input is valid UTF-8 representing only Unicode code points from"] # [doc = " U+0000 to U+00FF, inclusive, converts the input into output that"] # [doc = " represents the value of each code point as the unsigned byte value of"] # [doc = " each output byte."] # [doc = ""] # [doc = " If the input does not fulfill the condition stated above, this function"] # [doc = " panics if debug assertions are enabled (and fuzzing isn't) and otherwise"] # [doc = " does something that is memory-safe without any promises about any"] # [doc = " properties of the output. In particular, callers shouldn't assume the"] # [doc = " output to be the same across crate versions or CPU architectures and"] # [doc = " should not assume that non-ASCII input can't map to ASCII output."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] # [doc = ""] # [doc = " If debug assertions are enabled (and not fuzzing) and the input is"] # [doc = " not in the range U+0000 to U+00FF, inclusive."] pub fn convert_utf8_to_latin1_lossy (src : & [u8] , dst : & mut [u8]) -> usize { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; non_fuzz_debug_assert ! (is_utf8_latin1 (src)) ; let src_len = src . len () ; let src_ptr = src . as_ptr () ; let dst_ptr = dst . as_mut_ptr () ; let mut total_read = 0usize ; let mut total_written = 0usize ; loop { let src_left = src_len - total_read ; if let Some ((non_ascii , consumed)) = unsafe { ascii_to_ascii (src_ptr . add (total_read) , dst_ptr . add (total_written) , src_left ,) } { total_read += consumed + 1 ; total_written += consumed ; if total_read == src_len { return total_written ; } let trail = src [total_read] ; total_read += 1 ; dst [total_written] = ((non_ascii & 0x1F) << 6) | (trail & 0x3F) ; total_written += 1 ; continue ; } return total_written + src_left ; } }
    };
}

convert_utf8_to_latin1_lossy!()