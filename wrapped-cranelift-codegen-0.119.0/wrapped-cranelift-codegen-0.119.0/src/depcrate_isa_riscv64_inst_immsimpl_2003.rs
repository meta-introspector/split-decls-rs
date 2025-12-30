// Generated macro for impl_2003 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2003 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2003"}
// Dependencies: {}
impl Imm12 { pub (crate) const ZERO : Self = Self { bits : 0 } ; pub (crate) const ONE : Self = Self { bits : 1 } ; pub fn maybe_from_u64 (val : u64) -> Option < Imm12 > { Self :: maybe_from_i64 (val as i64) } pub fn maybe_from_i64 (val : i64) -> Option < Imm12 > { if val >= - 2048 && val <= 2047 { Some (Imm12 { bits : val as u16 & 0xfff , }) } else { None } } # [inline] pub fn from_i16 (bits : i16) -> Self { assert ! (bits >= - 2048 && bits <= 2047) ; Self { bits : (bits & 0xfff) as u16 , } } # [inline] pub fn as_i16 (self) -> i16 { (self . bits << 4) as i16 >> 4 } # [inline] pub fn bits (& self) -> u32 { self . bits . into () } }
};
}
