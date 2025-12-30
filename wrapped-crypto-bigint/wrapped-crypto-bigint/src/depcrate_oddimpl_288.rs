// Generated macro for impl_288 (impl)
macro_rules! Depcrate_oddimpl_288 {
() => {
// Module: crate::odd
// Provides: {"impl_288"}
// Dependencies: {}
# [doc = " Any odd integer multiplied by another odd integer is definitionally odd."] impl < T > Mul < Self > for Odd < T > where T : Mul < T , Output = T > , { type Output = Self ; fn mul (self , rhs : Self) -> Self { Self (self . 0 * rhs . 0) } }
};
}
