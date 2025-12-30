// Generated macro for impl_580 (impl)
macro_rules! Depcrate_dynamic_errorimpl_580 {
() => {
// Module: crate::dynamic::error
// Provides: {"impl_580"}
// Dependencies: {}
impl < T : Into < String > > From < T > for SchemaError { fn from (err : T) -> Self { SchemaError (err . into ()) } }
};
}
