// Generated macro for impl_46 (impl)
macro_rules! Depcrate_doubleimpl_46 {
() => {
// Module: crate::double
// Provides: {"impl_46"}
// Dependencies: {}
impl Add for udouble { type Output = udouble ; # [inline] fn add (self , rhs : Self) -> Self :: Output { let (lo , carry) = self . lo . overflowing_add (rhs . lo) ; let hi = self . hi + rhs . hi + carry as umax ; Self { lo , hi } } }
};
}
