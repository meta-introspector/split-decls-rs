// Generated macro for impl_229 (impl)
macro_rules! Depcrate_non_zeroimpl_229 {
() => {
// Module: crate::non_zero
// Provides: {"impl_229"}
// Dependencies: {}
# [doc = " Any non-zero integer multiplied by another non-zero integer is definitionally non-zero."] impl < T > Mul < Self > for NonZero < T > where T : Mul < T , Output = T > , { type Output = Self ; fn mul (self , rhs : Self) -> Self { Self (self . 0 * rhs . 0) } }
};
}
