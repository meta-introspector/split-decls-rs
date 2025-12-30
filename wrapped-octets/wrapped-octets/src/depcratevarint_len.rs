// Generated macro for varint_len (function)
macro_rules! Depcratevarint_len {
() => {
// Module: crate
// Provides: {"varint_len"}
// Dependencies: {}
# [doc = " Returns how many bytes it would take to encode `v` as a variable-length"] # [doc = " integer."] pub const fn varint_len (v : u64) -> usize { if v <= 63 { 1 } else if v <= 16383 { 2 } else if v <= 1_073_741_823 { 4 } else if v <= MAX_VAR_INT { 8 } else { unreachable ! () } }
};
}
