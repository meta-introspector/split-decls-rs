// Generated macro for impl_323 (impl)
macro_rules! Depcrate_paramsimpl_323 {
() => {
// Module: crate::params
// Provides: {"impl_323"}
// Dependencies: {}
impl < S : BindIndex , T : ToSql > Params for & [(S , T)] { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . bind_parameters_named (self) } }
};
}
