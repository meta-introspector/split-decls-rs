// Generated macro for impl_1190 (impl)
macro_rules! Depcrate_units_convertibleimpl_1190 {
() => {
// Module: crate::units::convertible
// Provides: {"impl_1190"}
// Dependencies: {}
impl Convertible for Ratio < BigInt > { fn mul_refs (& self , other : & Self) -> Self { self * other } fn add_refs (& self , other : & Self) -> Self { self + other } fn from_ratio_bigint (ratio : Ratio < BigInt >) -> Option < Self > { Some (ratio) } fn reciprocal (& self) -> Self { self . recip () } }
};
}
