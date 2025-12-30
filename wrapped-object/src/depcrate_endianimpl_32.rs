// Generated macro for impl_32 (impl)
macro_rules! Depcrate_endianimpl_32 {
() => {
// Module: crate::endian
// Provides: {"impl_32"}
// Dependencies: {}
impl Endian for Endianness { # [inline] fn from_big_endian (big_endian : bool) -> Option < Self > { Some (if big_endian { Endianness :: Big } else { Endianness :: Little }) } # [inline] fn is_big_endian (self) -> bool { self != Endianness :: Little } }
};
}
