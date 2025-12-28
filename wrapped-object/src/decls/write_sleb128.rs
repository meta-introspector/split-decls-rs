macro_rules! write_sleb128 {
    () => {
        # [doc = " Write a signed number using the LEB128 encoding to a buffer."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [allow (dead_code)] pub (crate) fn write_sleb128 (buf : & mut Vec < u8 > , mut val : i64) -> usize { let mut len = 0 ; loop { let mut byte = val as u8 ; val >>= 6 ; let done = val == 0 || val == - 1 ; if done { byte &= ! 0x80 ; } else { val >>= 1 ; byte |= 0x80 ; } buf . push (byte) ; len += 1 ; if done { return len ; } } }
    };
}

write_sleb128!()