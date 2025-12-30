// Generated macro for impl_2011 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2011 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2011"}
// Dependencies: {}
impl UImm5 { # [doc = " Create an unsigned 5-bit immediate from u8."] pub fn maybe_from_u8 (value : u8) -> Option < UImm5 > { if value < 32 { Some (UImm5 { value }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { u32 :: from (self . value) } }
};
}
