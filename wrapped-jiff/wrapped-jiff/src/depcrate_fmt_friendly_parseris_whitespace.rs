// Generated macro for is_whitespace (function)
macro_rules! Depcrate_fmt_friendly_parseris_whitespace {
() => {
// Module: crate::fmt::friendly::parser
// Provides: {"is_whitespace"}
// Dependencies: {}
# [doc = " Returns true if the byte is ASCII whitespace."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn is_whitespace (byte : & u8) -> bool { matches ! (* byte , b' ' | b'\t' | b'\n' | b'\r' | b'\x0C') }
};
}
