// Generated macro for impl_2014 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2014 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2014"}
// Dependencies: {}
impl Imm5 { # [doc = " Create an signed 5-bit immediate from an i8."] pub fn maybe_from_i8 (value : i8) -> Option < Imm5 > { if value >= - 16 && value <= 15 { Some (Imm5 { value }) } else { None } } pub fn from_bits (value : u8) -> Imm5 { assert_eq ! (value & 0x1f , value) ; let signed = ((value << 3) as i8) >> 3 ; Imm5 { value : signed } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u8 { self . value as u8 & 0x1f } }
};
}
