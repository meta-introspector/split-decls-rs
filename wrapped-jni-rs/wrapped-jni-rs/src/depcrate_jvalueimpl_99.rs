// Generated macro for impl_99 (impl)
macro_rules! Depcrate_jvalueimpl_99 {
() => {
// Module: crate::jvalue
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'local , T : Into < JObject < 'local > > > From < T > for JValueOwned < 'local > { fn from (other : T) -> Self { Self :: Object (other . into ()) } }
};
}
