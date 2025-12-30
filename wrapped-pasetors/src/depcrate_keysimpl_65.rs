// Generated macro for impl_65 (impl)
macro_rules! Depcrate_keysimpl_65 {
() => {
// Module: crate::keys
// Provides: {"impl_65"}
// Dependencies: {}
impl < V : Version > PartialEq < SymmetricKey < V > > for SymmetricKey < V > { fn eq (& self , other : & SymmetricKey < V >) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (other . as_bytes ()) . into () } }
};
}
