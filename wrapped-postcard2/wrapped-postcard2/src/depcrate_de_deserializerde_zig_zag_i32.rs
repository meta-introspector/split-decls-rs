// Generated macro for de_zig_zag_i32 (function)
macro_rules! Depcrate_de_deserializerde_zig_zag_i32 {
() => {
// Module: crate::de::deserializer
// Provides: {"de_zig_zag_i32"}
// Dependencies: {}
fn de_zig_zag_i32 (n : u32) -> i32 { ((n >> 1) as i32) ^ (- ((n & 0b1) as i32)) }
};
}
