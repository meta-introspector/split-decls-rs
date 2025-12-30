// Generated macro for impl_49 (impl)
macro_rules! Depcrate_doubleimpl_49 {
() => {
// Module: crate::double
// Provides: {"impl_49"}
// Dependencies: {}
impl AddAssign < umax > for udouble { # [inline] fn add_assign (& mut self , rhs : umax) { let (lo , carry) = self . lo . overflowing_add (rhs) ; self . lo = lo ; if carry { self . hi += 1 } } }
};
}
