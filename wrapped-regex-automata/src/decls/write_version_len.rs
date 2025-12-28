macro_rules! write_version_len {
    () => {
        # [doc = " Returns the number of bytes written by writing the version number."] pub (crate) fn write_version_len () -> usize { size_of :: < u32 > () }
    };
}

write_version_len!();