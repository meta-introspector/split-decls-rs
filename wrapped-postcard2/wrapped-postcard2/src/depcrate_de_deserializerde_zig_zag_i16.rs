// Generated macro for de_zig_zag_i16 (function)
macro_rules! Depcrate_de_deserializerde_zig_zag_i16 {
() => {
// Module: crate::de::deserializer
// Provides: {"de_zig_zag_i16"}
// Dependencies: {}
fn de_zig_zag_i16 (n : u16) -> i16 { ((n >> 1) as i16) ^ (- ((n & 0b1) as i16)) }
};
}
