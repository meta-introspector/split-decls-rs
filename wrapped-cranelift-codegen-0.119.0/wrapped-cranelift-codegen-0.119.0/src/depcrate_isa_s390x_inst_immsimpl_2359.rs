// Generated macro for impl_2359 (impl)
macro_rules! Depcrate_isa_s390x_inst_immsimpl_2359 {
() => {
// Module: crate::isa::s390x::inst::imms
// Provides: {"impl_2359"}
// Dependencies: {}
impl SImm20 { pub fn maybe_from_i64 (value : i64) -> Option < SImm20 > { if value >= - 524288 && value < 524288 { Some (SImm20 { value : value as i32 , }) } else { None } } pub fn from_uimm12 (value : UImm12) -> SImm20 { SImm20 { value : value . bits () as i32 , } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { let encoded : u32 = self . value as u32 ; encoded & 0xfffff } }
};
}
