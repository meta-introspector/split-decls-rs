// Generated macro for impl_1211 (impl)
macro_rules! Depcrate_isa_x64_encoding_eveximpl_1211 {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"impl_1211"}
// Dependencies: {}
impl EvexVectorLength { # [doc = " Encode the `L'` and `L` bits for merging with the P2 byte."] fn bits (& self) -> u8 { match self { Self :: V128 => 0b00 , Self :: V256 => 0b01 , Self :: V512 => 0b10 , } } }
};
}
