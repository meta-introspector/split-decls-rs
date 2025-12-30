// Generated macro for impl_25 (impl)
macro_rules! Depcrate_export_traitsimpl_25 {
() => {
// Module: crate::export::traits
// Provides: {"impl_25"}
// Dependencies: {}
impl < T > IntoResult for Option < T > { type Ok = T ; type Error = NoneError ; # [inline] fn into_result (self) -> Result < T , NoneError > { self . ok_or (NoneError) } }
};
}
