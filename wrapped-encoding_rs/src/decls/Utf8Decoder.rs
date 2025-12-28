macro_rules! Utf8Decoder {
    () => {
        pub struct Utf8Decoder { code_point : u32 , bytes_seen : usize , bytes_needed : usize , lower_boundary : u8 , upper_boundary : u8 , }
    };
}

Utf8Decoder!();