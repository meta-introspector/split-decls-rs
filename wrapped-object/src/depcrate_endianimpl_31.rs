// Generated macro for impl_31 (impl)
macro_rules! Depcrate_endianimpl_31 {
() => {
// Module: crate::endian
// Provides: {"impl_31"}
// Dependencies: {}
impl Default for Endianness { # [cfg (target_endian = "little")] # [inline] fn default () -> Endianness { Endianness :: Little } # [cfg (target_endian = "big")] # [inline] fn default () -> Endianness { Endianness :: Big } }
};
}
