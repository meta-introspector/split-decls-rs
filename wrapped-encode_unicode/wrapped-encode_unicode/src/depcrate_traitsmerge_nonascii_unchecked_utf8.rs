// Generated macro for merge_nonascii_unchecked_utf8 (function)
macro_rules! Depcrate_traitsmerge_nonascii_unchecked_utf8 {
() => {
// Module: crate::traits
// Provides: {"merge_nonascii_unchecked_utf8"}
// Dependencies: {}
# [doc = " Decodes the codepoint represented by a multi-byte UTF-8 sequence."] # [doc = ""] # [doc = " Does not check that the codepoint is valid,"] # [doc = " and returns `u32` because casting invalid codepoints to `char` is insta UB."] fn merge_nonascii_unchecked_utf8 (src : & [u8]) -> u32 { let mut c = src [0] as u32 & (0x7f >> src . len ()) ; for b in & src [1 ..] { c = (c << 6) | (b & 0b0011_1111) as u32 ; } c }
};
}
