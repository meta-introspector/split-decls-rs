// Generated macro for impl_1214 (impl)
macro_rules! Depcrate_isa_x64_encoding_eveximpl_1214 {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"impl_1214"}
// Dependencies: {}
impl EvexRoundingControl { # [doc = " Encode the `L'` and `L` bits for merging with the P2 byte."] fn bits (& self) -> u8 { match self { Self :: RNE => 0b00 , Self :: RD => 0b01 , Self :: RU => 0b10 , Self :: RZ => 0b11 , } } }
};
}
