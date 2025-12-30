// Generated macro for impl_238 (impl)
macro_rules! Depcrate_plots_minmaximpl_238 {
() => {
// Module: crate::plots::minmax
// Provides: {"impl_238"}
// Dependencies: {}
impl XMinMax { pub fn new (min : f32 , max : f32 , start : Option < f32 > , end : Option < f32 >) -> Self { let mut minmax = Self { min , max } ; if let Some (s) = start { minmax . min = s ; } if let Some (e) = end { minmax . max = minmax . max . min (e) ; } minmax } pub fn range (& self) -> std :: ops :: Range < f32 > { self . min .. self . max } }
};
}
