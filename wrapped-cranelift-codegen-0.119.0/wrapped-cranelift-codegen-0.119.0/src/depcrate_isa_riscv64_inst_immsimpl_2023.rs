// Generated macro for impl_2023 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2023 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2023"}
// Dependencies: {}
impl Uimm5 { # [doc = " Create an unsigned 5-bit immediate from an u8"] pub fn maybe_from_u8 (value : u8) -> Option < Self > { if value <= 31 { Some (Self { value }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u8 { self . value & 0x1f } }
};
}
