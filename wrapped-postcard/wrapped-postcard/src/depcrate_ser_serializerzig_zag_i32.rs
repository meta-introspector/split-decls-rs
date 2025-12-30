// Generated macro for zig_zag_i32 (function)
macro_rules! Depcrate_ser_serializerzig_zag_i32 {
() => {
// Module: crate::ser::serializer
// Provides: {"zig_zag_i32"}
// Dependencies: {}
fn zig_zag_i32 (n : i32) -> u32 { ((n << 1) ^ (n >> 31)) as u32 }
};
}
