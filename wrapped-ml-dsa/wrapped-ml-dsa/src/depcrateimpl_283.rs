// Generated macro for impl_283 (impl)
macro_rules! Depcrateimpl_283 {
() => {
// Module: crate
// Provides: {"impl_283"}
// Dependencies: {}
impl < P : MlDsaParams > fmt :: Debug for KeyPair < P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("KeyPair") . field ("verifying_key" , & self . verifying_key) . finish_non_exhaustive () } }
};
}
