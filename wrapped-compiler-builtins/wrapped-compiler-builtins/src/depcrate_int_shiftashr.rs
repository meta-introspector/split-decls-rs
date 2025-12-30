// Generated macro for Ashr (trait)
macro_rules! Depcrate_int_shiftAshr {
() => {
// Module: crate::int::shift
// Provides: {"Ashr"}
// Dependencies: {}
trait Ashr : DInt { # [doc = " Returns arithmetic `a >> b`, requires `b < Self::BITS`"] fn ashr (self , shr : u32) -> Self { let n_h = Self :: H :: BITS ; if shr & n_h != 0 { Self :: from_lo_hi (self . hi () . wrapping_shr (shr - n_h) , self . hi () . wrapping_shr (n_h - 1) ,) } else if shr == 0 { self } else { Self :: from_lo_hi (self . lo () . logical_shr (shr) | self . hi () . wrapping_shl (n_h . wrapping_sub (shr)) , self . hi () . wrapping_shr (shr) ,) } } }
};
}
