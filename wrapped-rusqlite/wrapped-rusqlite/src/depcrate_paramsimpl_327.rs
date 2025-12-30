// Generated macro for impl_327 (impl)
macro_rules! Depcrate_paramsimpl_327 {
() => {
// Module: crate::params
// Provides: {"impl_327"}
// Dependencies: {}
impl < T : ToSql > Params for (T ,) { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . ensure_parameter_count (1) ? ; stmt . raw_bind_parameter (1 , self . 0) ? ; Ok (()) } }
};
}
