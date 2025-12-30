// Generated macro for impl_53 (impl)
macro_rules! Depcrate_doubleimpl_53 {
() => {
// Module: crate::double
// Provides: {"impl_53"}
// Dependencies: {}
impl SubAssign < umax > for udouble { # [inline] fn sub_assign (& mut self , rhs : umax) { let carry = self . lo < rhs ; self . lo = self . lo . wrapping_sub (rhs) ; if carry { self . hi -= 1 ; } } }
};
}
