// Generated macro for Ashl (trait)
macro_rules! Depcrate_int_shiftAshl {
() => {
// Module: crate::int::shift
// Provides: {"Ashl"}
// Dependencies: {}
trait Ashl : DInt { # [doc = " Returns `a << b`, requires `b < Self::BITS`"] fn ashl (self , shl : u32) -> Self { let n_h = Self :: H :: BITS ; if shl & n_h != 0 { self . lo () . wrapping_shl (shl - n_h) . widen_hi () } else if shl == 0 { self } else { Self :: from_lo_hi (self . lo () . wrapping_shl (shl) , self . lo () . logical_shr (n_h . wrapping_sub (shl)) | self . hi () . wrapping_shl (shl) ,) } } }
};
}
