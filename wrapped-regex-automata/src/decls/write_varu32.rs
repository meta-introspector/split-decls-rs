macro_rules! write_varu32 {
    () => {
        # [doc = " Write an unsigned 32-bit integer as a varint. In essence, `n` is written"] # [doc = " as a sequence of bytes where all bytes except for the last one have the"] # [doc = " most significant bit set. The least significant 7 bits correspond to the"] # [doc = " actual bits of `n`. So in the worst case, a varint uses 5 bytes, but in"] # [doc = " very common cases, it uses fewer than 4."] # [doc = ""] # [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn write_varu32 (data : & mut Vec < u8 > , mut n : u32) { while n >= 0b1000_0000 { data . push (n . low_u8 () | 0b1000_0000) ; n >>= 7 ; } data . push (n . low_u8 ()) ; }
    };
}

write_varu32!();