// Generated macro for impl_2361 (impl)
macro_rules! Depcrate_isa_s390x_inst_immsimpl_2361 {
() => {
// Module: crate::isa::s390x::inst::imms
// Provides: {"impl_2361"}
// Dependencies: {}
impl UImm16Shifted { # [doc = " Construct a UImm16Shifted from an arbitrary 64-bit constant if possible."] pub fn maybe_from_u64 (value : u64) -> Option < UImm16Shifted > { let mask0 = 0x0000_0000_0000_ffffu64 ; let mask1 = 0x0000_0000_ffff_0000u64 ; let mask2 = 0x0000_ffff_0000_0000u64 ; let mask3 = 0xffff_0000_0000_0000u64 ; if value == (value & mask0) { return Some (UImm16Shifted { bits : (value & mask0) as u16 , shift : 0 , }) ; } if value == (value & mask1) { return Some (UImm16Shifted { bits : ((value >> 16) & mask0) as u16 , shift : 1 , }) ; } if value == (value & mask2) { return Some (UImm16Shifted { bits : ((value >> 32) & mask0) as u16 , shift : 2 , }) ; } if value == (value & mask3) { return Some (UImm16Shifted { bits : ((value >> 48) & mask0) as u16 , shift : 3 , }) ; } None } pub fn maybe_with_shift (imm : u16 , shift : u8) -> Option < UImm16Shifted > { let shift_enc = shift / 16 ; if shift_enc > 3 { None } else { Some (UImm16Shifted { bits : imm , shift : shift_enc , }) } } pub fn negate_bits (& self) -> UImm16Shifted { UImm16Shifted { bits : ! self . bits , shift : self . shift , } } }
};
}
