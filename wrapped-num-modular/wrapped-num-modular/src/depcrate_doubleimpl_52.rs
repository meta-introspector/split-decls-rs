// Generated macro for impl_52 (impl)
macro_rules! Depcrate_doubleimpl_52 {
() => {
// Module: crate::double
// Provides: {"impl_52"}
// Dependencies: {}
impl SubAssign for udouble { # [inline] fn sub_assign (& mut self , rhs : Self) { let carry = self . lo < rhs . lo ; self . lo = self . lo . wrapping_sub (rhs . lo) ; self . hi -= rhs . hi + carry as umax ; } }
};
}
