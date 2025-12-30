// Generated macro for impl_2017 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2017 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2017"}
// Dependencies: {}
impl Imm6 { # [doc = " Create an signed 6-bit immediate from an i16"] pub fn maybe_from_i16 (value : i16) -> Option < Self > { if value >= - 32 && value <= 31 { Some (Self { value : value as i8 }) } else { None } } pub fn maybe_from_i32 (value : i32) -> Option < Self > { value . try_into () . ok () . and_then (Imm6 :: maybe_from_i16) } pub fn maybe_from_imm12 (value : Imm12) -> Option < Self > { Imm6 :: maybe_from_i16 (value . as_i16 ()) } # [doc = " Bits for encoding."] pub fn bits (& self) -> u8 { self . value as u8 & 0x3f } }
};
}
