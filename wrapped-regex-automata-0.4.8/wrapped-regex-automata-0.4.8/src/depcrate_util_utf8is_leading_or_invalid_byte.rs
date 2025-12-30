// Generated macro for is_leading_or_invalid_byte (function)
macro_rules! Depcrate_util_utf8is_leading_or_invalid_byte {
() => {
// Module: crate::util::utf8
// Provides: {"is_leading_or_invalid_byte"}
// Dependencies: {}
# [doc = " Returns true if and only if the given byte is either a valid leading UTF-8"] # [doc = " byte, or is otherwise an invalid byte that can never appear anywhere in a"] # [doc = " valid UTF-8 sequence."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn is_leading_or_invalid_byte (b : u8) -> bool { (b & 0b1100_0000) != 0b1000_0000 }
};
}
