// Generated macro for impl_161 (impl)
macro_rules! Depcrate_helpersimpl_161 {
() => {
// Module: crate::helpers
// Provides: {"impl_161"}
// Dependencies: {}
impl IntegerRoundings for i64 { fn div_ceil (self , rhs : Self) -> Self { let d = self / rhs ; let r = self % rhs ; if (r > 0 && rhs > 0) || (r < 0 && rhs < 0) { d + 1 } else { d } } }
};
}
