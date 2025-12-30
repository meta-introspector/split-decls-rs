// Generated macro for zig_zag_i128 (function)
macro_rules! Depcrate_ser_serializerzig_zag_i128 {
() => {
// Module: crate::ser::serializer
// Provides: {"zig_zag_i128"}
// Dependencies: {}
fn zig_zag_i128 (n : i128) -> u128 { ((n << 1) ^ (n >> 127)) as u128 }
};
}
