// Generated macro for impl_129 (impl)
macro_rules! Depcrate_asn1impl_129 {
() => {
// Module: crate::asn1
// Provides: {"impl_129"}
// Dependencies: {}
impl Asn1EnumeratedRef { # [doc = " Get the value, if it fits in the required bounds."] # [corresponds (ASN1_ENUMERATED_get_int64)] # [cfg (ossl110)] pub fn get_i64 (& self) -> Result < i64 , ErrorStack > { let mut crl_reason = 0 ; unsafe { cvt (ffi :: ASN1_ENUMERATED_get_int64 (& mut crl_reason , self . as_ptr () ,)) ? ; } Ok (crl_reason) } }
};
}
