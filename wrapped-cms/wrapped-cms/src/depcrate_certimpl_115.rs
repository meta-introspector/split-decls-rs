// Generated macro for impl_115 (impl)
macro_rules! Depcrate_certimpl_115 {
() => {
// Module: crate::cert
// Provides: {"impl_115"}
// Dependencies: {}
impl ValueOrd for CertificateChoices { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { use der :: DerOrd ; use der :: Encode ; self . to_der () ? . der_cmp (& other . to_der () ?) } }
};
}
