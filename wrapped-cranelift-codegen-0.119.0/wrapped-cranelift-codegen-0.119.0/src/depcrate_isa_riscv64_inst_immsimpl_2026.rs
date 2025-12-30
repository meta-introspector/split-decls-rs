// Generated macro for impl_2026 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2026 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2026"}
// Dependencies: {}
impl Uimm2 { # [doc = " Create an unsigned 2-bit immediate from an u8"] pub fn maybe_from_u8 (value : u8) -> Option < Self > { if value <= 3 { Some (Self { value }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u8 { self . value & 0x3 } }
};
}
