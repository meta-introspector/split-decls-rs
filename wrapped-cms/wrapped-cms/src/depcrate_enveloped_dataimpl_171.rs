// Generated macro for impl_171 (impl)
macro_rules! Depcrate_enveloped_dataimpl_171 {
() => {
// Module: crate::enveloped_data
// Provides: {"impl_171"}
// Dependencies: {}
impl ValueOrd for RecipientInfo { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { use der :: DerOrd ; use der :: Encode ; self . to_der () ? . der_cmp (& other . to_der () ?) } }
};
}
