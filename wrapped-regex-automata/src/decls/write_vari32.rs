macro_rules! write_vari32 {
    () => {
        # [doc = " Write a signed 32-bit integer using zig-zag encoding."] # [doc = ""] # [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn write_vari32 (data : & mut Vec < u8 > , n : i32) { let mut un = n . to_bits () << 1 ; if n < 0 { un = ! un ; } write_varu32 (data , un) }
    };
}

write_vari32!()