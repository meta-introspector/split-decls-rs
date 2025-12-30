// Generated macro for Mul (trait)
macro_rules! Depcrate_int_mulMul {
() => {
// Module: crate::int::mul
// Provides: {"Mul"}
// Dependencies: {}
trait Mul : DInt + Int where Self :: H : DInt , { fn mul (self , rhs : Self) -> Self { let lhs_lo = self . lo () ; let rhs_lo = rhs . lo () ; let tmp_0 = lhs_lo . lo () . zero_widen_mul (rhs_lo . lo ()) ; let tmp_1 = lhs_lo . lo () . zero_widen_mul (rhs_lo . hi ()) ; let tmp_2 = lhs_lo . hi () . zero_widen_mul (rhs_lo . lo ()) ; let tmp_3 = lhs_lo . hi () . zero_widen_mul (rhs_lo . hi ()) ; let mul = Self :: from_lo_hi (tmp_0 , tmp_3) . wrapping_add (tmp_1 . zero_widen () << (Self :: BITS / 4)) . wrapping_add (tmp_2 . zero_widen () << (Self :: BITS / 4)) ; mul . wrapping_add (lhs_lo . wrapping_mul (rhs . hi ()) . widen_hi ()) . wrapping_add (self . hi () . wrapping_mul (rhs_lo) . widen_hi ()) } }
};
}
