// Generated macro for impl_1453 (impl)
macro_rules! Depcrate_write_peimpl_1453 {
() => {
// Module: crate::write::pe
// Provides: {"impl_1453"}
// Dependencies: {}
impl RelocBlock { fn size (& self) -> u32 { mem :: size_of :: < pe :: ImageBaseRelocation > () as u32 + self . count * mem :: size_of :: < u16 > () as u32 } }
};
}
