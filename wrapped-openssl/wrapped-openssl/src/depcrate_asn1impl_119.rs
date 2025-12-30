// Generated macro for impl_119 (impl)
macro_rules! Depcrate_asn1impl_119 {
() => {
// Module: crate::asn1
// Provides: {"impl_119"}
// Dependencies: {}
impl Asn1OctetString { # [doc = " Creates an Asn1OctetString from bytes"] pub fn new_from_bytes (value : & [u8]) -> Result < Self , ErrorStack > { ffi :: init () ; unsafe { let s = cvt_p (ffi :: ASN1_OCTET_STRING_new ()) ? ; ffi :: ASN1_OCTET_STRING_set (s , value . as_ptr () , value . len () . try_into () . unwrap ()) ; Ok (Self :: from_ptr (s)) } } }
};
}
