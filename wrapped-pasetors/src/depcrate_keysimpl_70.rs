// Generated macro for impl_70 (impl)
macro_rules! Depcrate_keysimpl_70 {
() => {
// Module: crate::keys
// Provides: {"impl_70"}
// Dependencies: {}
impl < V : Version > PartialEq < AsymmetricSecretKey < V > > for AsymmetricSecretKey < V > { fn eq (& self , other : & AsymmetricSecretKey < V >) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (other . as_bytes ()) . into () } }
};
}
