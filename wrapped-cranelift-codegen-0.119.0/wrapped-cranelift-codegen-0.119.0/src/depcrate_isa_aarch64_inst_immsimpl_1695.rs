// Generated macro for impl_1695 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1695 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1695"}
// Dependencies: {}
impl Imm12 { # [doc = " Compute a Imm12 from raw bits, if possible."] pub fn maybe_from_u64 (val : u64) -> Option < Imm12 > { if val & ! 0xfff == 0 { Some (Imm12 { bits : val as u16 , shift12 : false , }) } else if val & ! (0xfff << 12) == 0 { Some (Imm12 { bits : (val >> 12) as u16 , shift12 : true , }) } else { None } } # [doc = " Bits for 2-bit \"shift\" field in e.g. AddI."] pub fn shift_bits (& self) -> u32 { if self . shift12 { 0b01 } else { 0b00 } } # [doc = " Bits for 12-bit \"imm\" field in e.g. AddI."] pub fn imm_bits (& self) -> u32 { self . bits as u32 } # [doc = " Get the actual value that this immediate corresponds to."] pub fn value (& self) -> u32 { let base = self . bits as u32 ; if self . shift12 { base << 12 } else { base } } }
};
}
