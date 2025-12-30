// Generated macro for impl_51 (impl)
macro_rules! Depcrate_doubleimpl_51 {
() => {
// Module: crate::double
// Provides: {"impl_51"}
// Dependencies: {}
impl Sub < umax > for udouble { type Output = Self ; # [inline] fn sub (self , rhs : umax) -> Self :: Output { let carry = self . lo < rhs ; let lo = self . lo . wrapping_sub (rhs) ; let hi = if carry { self . hi - 1 } else { self . hi } ; Self { lo , hi } } }
};
}
