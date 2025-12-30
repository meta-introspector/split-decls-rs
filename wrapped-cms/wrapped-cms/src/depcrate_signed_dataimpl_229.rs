// Generated macro for impl_229 (impl)
macro_rules! Depcrate_signed_dataimpl_229 {
() => {
// Module: crate::signed_data
// Provides: {"impl_229"}
// Dependencies: {}
impl ValueOrd for SignerIdentifier { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { use der :: Encode ; self . to_der () ? . der_cmp (& other . to_der () ?) } }
};
}
