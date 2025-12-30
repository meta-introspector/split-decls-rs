// Generated macro for rank (function)
macro_rules! Depcrate_hir_literalrank {
() => {
// Module: crate::hir::literal
// Provides: {"rank"}
// Dependencies: {}
# [doc = " Returns the \"rank\" of the given byte."] # [doc = ""] # [doc = " The minimum rank value is `0` and the maximum rank value is `255`."] # [doc = ""] # [doc = " The rank of a byte is derived from a heuristic background distribution of"] # [doc = " relative frequencies of bytes. The heuristic says that lower the rank of a"] # [doc = " byte, the less likely that byte is to appear in any arbitrary haystack."] pub fn rank (byte : u8) -> u8 { crate :: rank :: BYTE_FREQUENCIES [usize :: from (byte)] }
};
}
