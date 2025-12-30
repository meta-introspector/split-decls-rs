// Generated macro for encode_vlq (function)
macro_rules! Depcrate_encodeencode_vlq {
() => {
// Module: crate::encode
// Provides: {"encode_vlq"}
// Dependencies: {}
# [doc = " Write a git-flavoured VLQ value into `buf`."] # [doc = ""] # [doc = " Returns the slice within `buf` that holds the value."] fn encode_vlq (mut val : usize , buf : & mut [u8 ; 4]) -> & [u8] { macro_rules ! step { ($ n : expr) => { buf [$ n] = if $ n == 3 { (val & (VAL_MASK as usize)) as u8 } else { val -= 1 ; NEXT_MASK | (val & (VAL_MASK as usize)) as u8 } ; val >>= 7 ; if val == 0 { return & buf [$ n ..] ; } } ; } step ! (3) ; step ! (2) ; step ! (1) ; step ! (0) ; panic ! ("integer is too big") }
};
}
