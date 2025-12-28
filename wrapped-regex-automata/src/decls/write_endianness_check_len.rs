macro_rules! write_endianness_check_len {
    () => {
        # [doc = " Returns the number of bytes written by the endianness check."] pub (crate) fn write_endianness_check_len () -> usize { size_of :: < u32 > () }
    };
}

write_endianness_check_len!()