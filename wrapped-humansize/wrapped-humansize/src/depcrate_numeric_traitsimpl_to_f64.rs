// Generated macro for impl_to_f64 (macro)
macro_rules! Depcrate_numeric_traitsimpl_to_f64 {
() => {
// Module: crate::numeric_traits
// Provides: {"impl_to_f64"}
// Dependencies: {}
macro_rules ! impl_to_f64 { (for $ ($ t : ty) *) => ($ (impl ToF64 for $ t { fn to_f64 (& self) -> f64 { * self as f64 } }) *) }
};
}
