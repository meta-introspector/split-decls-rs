// Generated macro for impl_48 (impl)
macro_rules! Depcrate_doubleimpl_48 {
() => {
// Module: crate::double
// Provides: {"impl_48"}
// Dependencies: {}
impl AddAssign for udouble { # [inline] fn add_assign (& mut self , rhs : Self) { let (lo , carry) = self . lo . overflowing_add (rhs . lo) ; self . lo = lo ; self . hi += rhs . hi + carry as umax ; } }
};
}
