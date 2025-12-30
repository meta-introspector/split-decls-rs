// Generated macro for Utf8Sequence (enum)
macro_rules! Depcrate_utf8Utf8Sequence {
() => {
// Module: crate::utf8
// Provides: {"Utf8Sequence"}
// Dependencies: {}
# [doc = " Utf8Sequence represents a sequence of byte ranges."] # [doc = ""] # [doc = " To match a Utf8Sequence, a candidate byte sequence must match each"] # [doc = " successive range."] # [doc = ""] # [doc = " For example, if there are two ranges, `[C2-DF][80-BF]`, then the byte"] # [doc = " sequence `\\xDD\\x61` would not match because `0x61 < 0x80`."] # [derive (Copy , Clone , Eq , PartialEq , PartialOrd , Ord)] pub enum Utf8Sequence { # [doc = " One byte range."] One (Utf8Range) , # [doc = " Two successive byte ranges."] Two ([Utf8Range ; 2]) , # [doc = " Three successive byte ranges."] Three ([Utf8Range ; 3]) , # [doc = " Four successive byte ranges."] Four ([Utf8Range ; 4]) , }
};
}
