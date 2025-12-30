// Generated macro for impl_1209 (impl)
macro_rules! Depcrate_isa_x64_encoding_eveximpl_1209 {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"impl_1209"}
// Dependencies: {}
impl EvexContext { # [doc = " Encode the `L'`, `L`, and `b` bits (bits 6:4 of EVEX P2 byte) for merging with the P2 byte."] pub fn bits (& self) -> u8 { match self { Self :: RoundingRegToRegFP { rc } => 0b001 | rc . bits () << 1 , Self :: NoRoundingFP { sae , length } => (* sae as u8) | length . bits () << 1 , Self :: MemoryOp { broadcast , length } => (* broadcast as u8) | length . bits () << 1 , Self :: Other { length } => length . bits () << 1 , } } }
};
}
