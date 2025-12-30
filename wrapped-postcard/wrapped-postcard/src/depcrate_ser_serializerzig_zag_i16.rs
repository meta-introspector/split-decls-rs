// Generated macro for zig_zag_i16 (function)
macro_rules! Depcrate_ser_serializerzig_zag_i16 {
() => {
// Module: crate::ser::serializer
// Provides: {"zig_zag_i16"}
// Dependencies: {}
fn zig_zag_i16 (n : i16) -> u16 { ((n << 1) ^ (n >> 15)) as u16 }
};
}
