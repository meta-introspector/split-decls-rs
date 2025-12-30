// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_isa_x64_encoding_eveximpl_1217 {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"impl_1217"}
// Dependencies: {}
impl EvexMasking { # [doc = " Encode the `z` bit for merging with the P2 byte."] pub fn z_bit (& self) -> u8 { match self { Self :: None | Self :: Merging { .. } => 0 , Self :: Zeroing { .. } => 1 , } } # [doc = " Encode the `aaa` bits for merging with the P2 byte."] pub fn aaa_bits (& self) -> u8 { match self { Self :: None => 0b000 , Self :: Merging { k } | Self :: Zeroing { k } => { debug_assert ! (* k <= 7) ; * k } } } }
};
}
