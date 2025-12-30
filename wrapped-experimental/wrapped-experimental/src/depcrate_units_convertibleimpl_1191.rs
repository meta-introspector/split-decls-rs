// Generated macro for impl_1191 (impl)
macro_rules! Depcrate_units_convertibleimpl_1191 {
() => {
// Module: crate::units::convertible
// Provides: {"impl_1191"}
// Dependencies: {}
impl Convertible for f64 { fn mul_refs (& self , other : & Self) -> Self { self * other } fn add_refs (& self , other : & Self) -> Self { self + other } fn from_ratio_bigint (ratio : Ratio < BigInt >) -> Option < Self > { ratio . to_f64 () } fn reciprocal (& self) -> Self { self . recip () } }
};
}
