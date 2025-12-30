// Generated macro for impl_47 (impl)
macro_rules! Depcrate_doubleimpl_47 {
() => {
// Module: crate::double
// Provides: {"impl_47"}
// Dependencies: {}
impl Add < umax > for udouble { type Output = udouble ; # [inline] fn add (self , rhs : umax) -> Self :: Output { let (lo , carry) = self . lo . overflowing_add (rhs) ; let hi = if carry { self . hi + 1 } else { self . hi } ; Self { lo , hi } } }
};
}
