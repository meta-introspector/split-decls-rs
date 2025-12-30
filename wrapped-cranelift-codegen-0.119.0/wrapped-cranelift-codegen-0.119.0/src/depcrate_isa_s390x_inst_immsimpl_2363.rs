// Generated macro for impl_2363 (impl)
macro_rules! Depcrate_isa_s390x_inst_immsimpl_2363 {
() => {
// Module: crate::isa::s390x::inst::imms
// Provides: {"impl_2363"}
// Dependencies: {}
impl UImm32Shifted { # [doc = " Construct a UImm32Shifted from an arbitrary 64-bit constant if possible."] pub fn maybe_from_u64 (value : u64) -> Option < UImm32Shifted > { let mask0 = 0x0000_0000_ffff_ffffu64 ; let mask1 = 0xffff_ffff_0000_0000u64 ; if value == (value & mask0) { return Some (UImm32Shifted { bits : (value & mask0) as u32 , shift : 0 , }) ; } if value == (value & mask1) { return Some (UImm32Shifted { bits : ((value >> 32) & mask0) as u32 , shift : 1 , }) ; } None } pub fn maybe_with_shift (imm : u32 , shift : u8) -> Option < UImm32Shifted > { let shift_enc = shift / 32 ; if shift_enc > 3 { None } else { Some (UImm32Shifted { bits : imm , shift : shift_enc , }) } } pub fn negate_bits (& self) -> UImm32Shifted { UImm32Shifted { bits : ! self . bits , shift : self . shift , } } }
};
}
