// Generated macro for impl_1701 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1701 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1701"}
// Dependencies: {}
impl MoveWideConst { # [doc = " Construct a MoveWideConst from an arbitrary 64-bit constant if possible."] pub fn maybe_from_u64 (value : u64) -> Option < MoveWideConst > { let mask0 = 0x0000_0000_0000_ffffu64 ; let mask1 = 0x0000_0000_ffff_0000u64 ; let mask2 = 0x0000_ffff_0000_0000u64 ; let mask3 = 0xffff_0000_0000_0000u64 ; if value == (value & mask0) { return Some (MoveWideConst { bits : (value & mask0) as u16 , shift : 0 , }) ; } if value == (value & mask1) { return Some (MoveWideConst { bits : ((value >> 16) & mask0) as u16 , shift : 1 , }) ; } if value == (value & mask2) { return Some (MoveWideConst { bits : ((value >> 32) & mask0) as u16 , shift : 2 , }) ; } if value == (value & mask3) { return Some (MoveWideConst { bits : ((value >> 48) & mask0) as u16 , shift : 3 , }) ; } None } # [doc = " Create a `MoveWideCosnt` from a given shift, if possible."] pub fn maybe_with_shift (imm : u16 , shift : u8) -> Option < MoveWideConst > { let shift_enc = shift / 16 ; if shift_enc > 3 { None } else { Some (MoveWideConst { bits : imm , shift : shift_enc , }) } } # [doc = " Create a zero immediate of this format."] pub fn zero () -> MoveWideConst { MoveWideConst { bits : 0 , shift : 0 } } }
};
}
