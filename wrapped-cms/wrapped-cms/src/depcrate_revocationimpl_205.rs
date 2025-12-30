// Generated macro for impl_205 (impl)
macro_rules! Depcrate_revocationimpl_205 {
() => {
// Module: crate::revocation
// Provides: {"impl_205"}
// Dependencies: {}
impl ValueOrd for RevocationInfoChoice { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { use der :: DerOrd ; use der :: Encode ; self . to_der () ? . der_cmp (& other . to_der () ?) } }
};
}
