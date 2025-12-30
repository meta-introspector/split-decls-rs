// Generated macro for impl_319 (impl)
macro_rules! Depcrate_paramsimpl_319 {
() => {
// Module: crate::params
// Provides: {"impl_319"}
// Dependencies: {}
impl Params for [& (dyn ToSql + Send + Sync) ; 0] { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . ensure_parameter_count (0) } }
};
}
