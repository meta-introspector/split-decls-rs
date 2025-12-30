// Generated macro for impl_1740 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1740 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1740"}
// Dependencies: {}
impl BranchTarget { # [doc = " Return the target's label, if it is a label-based target."] pub fn as_label (self) -> Option < MachLabel > { match self { BranchTarget :: Label (l) => Some (l) , _ => None , } } # [doc = " Return the target's offset, if specified, or zero if label-based."] pub fn as_offset14_or_zero (self) -> u32 { self . as_offset_bounded (14) } # [doc = " Return the target's offset, if specified, or zero if label-based."] pub fn as_offset19_or_zero (self) -> u32 { self . as_offset_bounded (19) } # [doc = " Return the target's offset, if specified, or zero if label-based."] pub fn as_offset26_or_zero (self) -> u32 { self . as_offset_bounded (26) } fn as_offset_bounded (self , bits : u32) -> u32 { let off = match self { BranchTarget :: ResolvedOffset (off) => off >> 2 , _ => 0 , } ; let hi = (1 << (bits - 1)) - 1 ; let lo = - (1 << bits - 1) ; assert ! (off <= hi) ; assert ! (off >= lo) ; (off as u32) & ((1 << bits) - 1) } }
};
}
