// Generated macro for impl_2020 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2020 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2020"}
// Dependencies: {}
impl Uimm6 { # [doc = " Create an unsigned 6-bit immediate from an u8"] pub fn maybe_from_u8 (value : u8) -> Option < Self > { if value <= 63 { Some (Self { value }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u8 { self . value & 0x3f } }
};
}
