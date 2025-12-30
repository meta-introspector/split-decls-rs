// Generated macro for impl_1260 (impl)
macro_rules! Depcrate_isa_x64_encoding_veximpl_1260 {
() => {
// Module: crate::isa::x64::encoding::vex
// Provides: {"impl_1260"}
// Dependencies: {}
impl VexVectorLength { # [doc = " Encode the `L` bit."] fn bits (& self) -> u8 { match self { Self :: V128 => 0b0 , Self :: V256 => 0b1 , } } }
};
}
