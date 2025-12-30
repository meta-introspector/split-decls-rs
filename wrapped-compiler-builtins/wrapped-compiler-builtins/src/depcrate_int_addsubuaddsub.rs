// Generated macro for UAddSub (trait)
macro_rules! Depcrate_int_addsubUAddSub {
() => {
// Module: crate::int::addsub
// Provides: {"UAddSub"}
// Dependencies: {}
trait UAddSub : DInt + Int { fn uadd (self , other : Self) -> Self { let (lo , carry) = self . lo () . overflowing_add (other . lo ()) ; let hi = self . hi () . wrapping_add (other . hi ()) ; let carry = if carry { Self :: H :: ONE } else { Self :: H :: ZERO } ; Self :: from_lo_hi (lo , hi . wrapping_add (carry)) } fn uadd_one (self) -> Self { let (lo , carry) = self . lo () . overflowing_add (Self :: H :: ONE) ; let carry = if carry { Self :: H :: ONE } else { Self :: H :: ZERO } ; Self :: from_lo_hi (lo , self . hi () . wrapping_add (carry)) } fn usub (self , other : Self) -> Self { let uneg = (! other) . uadd_one () ; self . uadd (uneg) } }
};
}
