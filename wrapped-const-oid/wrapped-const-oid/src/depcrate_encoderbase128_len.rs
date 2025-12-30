// Generated macro for base128_len (function)
macro_rules! Depcrate_encoderbase128_len {
() => {
// Module: crate::encoder
// Provides: {"base128_len"}
// Dependencies: {}
# [doc = " Compute the length of an arc when encoded in base 128."] const fn base128_len (arc : Arc) -> usize { match arc { 0 ..= 0x7f => 1 , 0x80 ..= 0x3fff => 2 , 0x4000 ..= 0x1fffff => 3 , 0x200000 ..= 0x0fffffff => 4 , _ => 5 , } }
};
}
