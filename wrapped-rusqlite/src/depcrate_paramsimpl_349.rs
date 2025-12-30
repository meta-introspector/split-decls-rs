// Generated macro for impl_349 (impl)
macro_rules! Depcrate_paramsimpl_349 {
() => {
// Module: crate::params
// Provides: {"impl_349"}
// Dependencies: {}
impl < I > Params for ParamsFromIter < I > where I : IntoIterator , I :: Item : ToSql , { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . bind_parameters (self . 0) } }
};
}
