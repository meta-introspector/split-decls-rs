// Generated macro for zig_zag_i64 (function)
macro_rules! Depcrate_ser_serializerzig_zag_i64 {
() => {
// Module: crate::ser::serializer
// Provides: {"zig_zag_i64"}
// Dependencies: {}
fn zig_zag_i64 (n : i64) -> u64 { ((n << 1) ^ (n >> 63)) as u64 }
};
}
