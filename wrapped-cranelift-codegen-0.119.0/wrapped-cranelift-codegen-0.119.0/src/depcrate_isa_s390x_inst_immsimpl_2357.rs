// Generated macro for impl_2357 (impl)
macro_rules! Depcrate_isa_s390x_inst_immsimpl_2357 {
() => {
// Module: crate::isa::s390x::inst::imms
// Provides: {"impl_2357"}
// Dependencies: {}
impl UImm12 { pub fn maybe_from_u64 (value : u64) -> Option < UImm12 > { if value < 4096 { Some (UImm12 { value : value as u16 , }) } else { None } } # [doc = " Create a zero immediate of this format."] pub fn zero () -> UImm12 { UImm12 { value : 0 } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { u32 :: from (self . value) } }
};
}
