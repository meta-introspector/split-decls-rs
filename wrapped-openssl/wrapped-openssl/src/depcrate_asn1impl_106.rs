// Generated macro for impl_106 (impl)
macro_rules! Depcrate_asn1impl_106 {
() => {
// Module: crate::asn1
// Provides: {"impl_106"}
// Dependencies: {}
impl Asn1Integer { # [doc = " Converts a bignum to an `Asn1Integer`."] # [doc = ""] # [doc = " Corresponds to [`BN_to_ASN1_INTEGER`]. Also see"] # [doc = " [`BigNumRef::to_asn1_integer`]."] # [doc = ""] # [doc = " [`BN_to_ASN1_INTEGER`]: https://docs.openssl.org/master/man3/BN_to_ASN1_INTEGER/"] # [doc = " [`BigNumRef::to_asn1_integer`]: ../bn/struct.BigNumRef.html#method.to_asn1_integer"] pub fn from_bn (bn : & BigNumRef) -> Result < Self , ErrorStack > { bn . to_asn1_integer () } }
};
}
