// Generated macro for impl_35 (impl)
macro_rules! Depcrate_endianimpl_35 {
() => {
// Module: crate::endian
// Provides: {"impl_35"}
// Dependencies: {}
impl Endian for LittleEndian { # [inline] fn from_big_endian (big_endian : bool) -> Option < Self > { if big_endian { None } else { Some (LittleEndian) } } # [inline] fn is_big_endian (self) -> bool { false } }
};
}
