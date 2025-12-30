// Generated macro for de_zig_zag_i64 (function)
macro_rules! Depcrate_de_deserializerde_zig_zag_i64 {
() => {
// Module: crate::de::deserializer
// Provides: {"de_zig_zag_i64"}
// Dependencies: {}
fn de_zig_zag_i64 (n : u64) -> i64 { ((n >> 1) as i64) ^ (- ((n & 0b1) as i64)) }
};
}
