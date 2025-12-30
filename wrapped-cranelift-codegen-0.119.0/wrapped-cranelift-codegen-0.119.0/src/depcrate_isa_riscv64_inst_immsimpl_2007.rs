// Generated macro for impl_2007 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2007 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2007"}
// Dependencies: {}
impl Imm20 { pub (crate) const ZERO : Self = Self { bits : 0 } ; pub fn maybe_from_u64 (val : u64) -> Option < Imm20 > { Self :: maybe_from_i64 (val as i64) } pub fn maybe_from_i64 (val : i64) -> Option < Imm20 > { if val >= - (0x7_ffff + 1) && val <= 0x7_ffff { Some (Imm20 { bits : val as u32 }) } else { None } } # [inline] pub fn from_i32 (bits : i32) -> Self { assert ! (bits >= - (0x7_ffff + 1) && bits <= 0x7_ffff) ; Self { bits : (bits as u32) & 0xf_ffff , } } # [inline] pub fn as_i32 (& self) -> i32 { ((self . bits << 12) as i32) >> 12 } # [inline] pub fn bits (& self) -> u32 { self . bits } }
};
}
