// Generated macro for de_zig_zag_i128 (function)
macro_rules! Depcrate_de_deserializerde_zig_zag_i128 {
() => {
// Module: crate::de::deserializer
// Provides: {"de_zig_zag_i128"}
// Dependencies: {}
fn de_zig_zag_i128 (n : u128) -> i128 { ((n >> 1) as i128) ^ (- ((n & 0b1) as i128)) }
};
}
