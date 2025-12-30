// Generated macro for impl_321 (impl)
macro_rules! Depcrate_paramsimpl_321 {
() => {
// Module: crate::params
// Provides: {"impl_321"}
// Dependencies: {}
impl Params for & [& dyn ToSql] { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . bind_parameters (self) } }
};
}
