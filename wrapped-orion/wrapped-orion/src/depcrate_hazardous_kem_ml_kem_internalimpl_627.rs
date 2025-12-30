// Generated macro for impl_627 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalimpl_627 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"impl_627"}
// Dependencies: {}
impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > PartialEq < DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > > for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn eq (& self , other : & DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke >) -> bool { use subtle :: ConstantTimeEq ; (self . unprotected_as_bytes () . ct_eq (other . unprotected_as_bytes ())) . into () } }
};
}
