// Generated macro for impl_1691 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1691 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1691"}
// Dependencies: {}
impl SImm9 { # [doc = " Create a signed 9-bit offset from a full-range value, if possible."] pub fn maybe_from_i64 (value : i64) -> Option < SImm9 > { if value >= - 256 && value <= 255 { Some (SImm9 { value : value as i16 , }) } else { None } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { (self . value as u32) & 0x1ff } # [doc = " Signed value of immediate."] pub fn value (& self) -> i32 { self . value as i32 } }
};
}
