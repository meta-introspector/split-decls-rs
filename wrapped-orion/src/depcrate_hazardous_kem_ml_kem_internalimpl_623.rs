// Generated macro for impl_623 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalimpl_623 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"impl_623"}
// Dependencies: {}
impl < const K : usize , const ENCODED_SIZE : usize , Pke : PkeParameters > PartialEq < & [u8] > for EncapKey < K , ENCODED_SIZE , Pke > { fn eq (& self , other : & & [u8]) -> bool { self . bytes == * other } }
};
}
