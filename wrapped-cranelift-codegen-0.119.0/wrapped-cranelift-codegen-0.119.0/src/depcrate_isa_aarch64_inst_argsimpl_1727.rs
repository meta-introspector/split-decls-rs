// Generated macro for impl_1727 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1727 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1727"}
// Dependencies: {}
impl ShiftOpShiftImm { # [doc = " Maximum shift for shifted-register operands."] pub const MAX_SHIFT : u64 = 63 ; # [doc = " Create a new shiftop shift amount, if possible."] pub fn maybe_from_shift (shift : u64) -> Option < ShiftOpShiftImm > { if shift <= Self :: MAX_SHIFT { Some (ShiftOpShiftImm (shift as u8)) } else { None } } # [doc = " Return the shift amount."] pub fn value (self) -> u8 { self . 0 } # [doc = " Mask down to a given number of bits."] pub fn mask (self , bits : u8) -> ShiftOpShiftImm { ShiftOpShiftImm (self . 0 & (bits - 1)) } }
};
}
