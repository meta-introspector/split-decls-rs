// Generated macro for impl_100 (impl)
macro_rules! Depcrate_jvalueimpl_100 {
() => {
// Module: crate::jvalue
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'obj_ref , T : AsRef < JObject < 'obj_ref > > > From < & 'obj_ref T > for JValue < 'obj_ref > { fn from (other : & 'obj_ref T) -> Self { Self :: Object (other . as_ref ()) } }
};
}
