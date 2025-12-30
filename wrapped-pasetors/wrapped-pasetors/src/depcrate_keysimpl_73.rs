// Generated macro for impl_73 (impl)
macro_rules! Depcrate_keysimpl_73 {
() => {
// Module: crate::keys
// Provides: {"impl_73"}
// Dependencies: {}
impl < V : Version > PartialEq < AsymmetricPublicKey < V > > for AsymmetricPublicKey < V > { fn eq (& self , other : & AsymmetricPublicKey < V >) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (other . as_bytes ()) . into () } }
};
}
