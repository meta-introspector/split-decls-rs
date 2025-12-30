// Generated macro for Lshr (trait)
macro_rules! Depcrate_int_shiftLshr {
() => {
// Module: crate::int::shift
// Provides: {"Lshr"}
// Dependencies: {}
trait Lshr : DInt { # [doc = " Returns logical `a >> b`, requires `b < Self::BITS`"] fn lshr (self , shr : u32) -> Self { let n_h = Self :: H :: BITS ; if shr & n_h != 0 { self . hi () . logical_shr (shr - n_h) . zero_widen () } else if shr == 0 { self } else { Self :: from_lo_hi (self . lo () . logical_shr (shr) | self . hi () . wrapping_shl (n_h . wrapping_sub (shr)) , self . hi () . logical_shr (shr) ,) } } }
};
}
