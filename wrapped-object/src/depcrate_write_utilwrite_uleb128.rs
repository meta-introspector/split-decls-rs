// Generated macro for write_uleb128 (function)
macro_rules! Depcrate_write_utilwrite_uleb128 {
() => {
// Module: crate::write::util
// Provides: {"write_uleb128"}
// Dependencies: {}
# [doc = " Write an unsigned number using the LEB128 encoding to a buffer."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [allow (dead_code)] pub (crate) fn write_uleb128 (buf : & mut Vec < u8 > , mut val : u64) -> usize { let mut len = 0 ; loop { let mut byte = (val & 0x7f) as u8 ; val >>= 7 ; let done = val == 0 ; if ! done { byte |= 0x80 ; } buf . push (byte) ; len += 1 ; if done { return len ; } } }
};
}
