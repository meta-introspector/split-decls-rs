macro_rules! write_uleb128 {
    () => {
        # [doc = " Write an unsigned number using the LEB128 encoding to a buffer."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [allow (dead_code)] pub (crate) fn write_uleb128 (buf : & mut Vec < u8 > , mut val : u64) -> usize { let mut len = 0 ; loop { let mut byte = (val & 0x7f) as u8 ; val >>= 7 ; let done = val == 0 ; if ! done { byte |= 0x80 ; } buf . push (byte) ; len += 1 ; if done { return len ; } } }
    };
}

write_uleb128!()