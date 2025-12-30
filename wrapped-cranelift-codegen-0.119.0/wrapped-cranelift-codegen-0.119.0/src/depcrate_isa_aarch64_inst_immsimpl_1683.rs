// Generated macro for impl_1683 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1683 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1683"}
// Dependencies: {}
impl UImm5 { # [doc = " Create an unsigned 5-bit immediate from u8."] pub fn maybe_from_u8 (value : u8) -> Option < UImm5 > { if value < 32 { Some (UImm5 { value }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { u32 :: from (self . value) } }
};
}
