// Generated macro for impl_629 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalimpl_629 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"impl_629"}
// Dependencies: {}
impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > PartialEq < & [u8] > for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn eq (& self , other : & & [u8]) -> bool { use subtle :: ConstantTimeEq ; (self . unprotected_as_bytes () . ct_eq (* other)) . into () } }
};
}
