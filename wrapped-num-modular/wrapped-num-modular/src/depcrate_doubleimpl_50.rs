// Generated macro for impl_50 (impl)
macro_rules! Depcrate_doubleimpl_50 {
() => {
// Module: crate::double
// Provides: {"impl_50"}
// Dependencies: {}
impl Sub for udouble { type Output = Self ; # [inline] fn sub (self , rhs : Self) -> Self :: Output { let carry = self . lo < rhs . lo ; let lo = self . lo . wrapping_sub (rhs . lo) ; let hi = self . hi - rhs . hi - carry as umax ; Self { lo , hi } } }
};
}
