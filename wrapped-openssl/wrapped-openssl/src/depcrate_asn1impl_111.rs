// Generated macro for impl_111 (impl)
macro_rules! Depcrate_asn1impl_111 {
() => {
// Module: crate::asn1
// Provides: {"impl_111"}
// Dependencies: {}
impl Asn1IntegerRef { # [allow (missing_docs , clippy :: unnecessary_cast)] # [deprecated (since = "0.10.6" , note = "use to_bn instead")] pub fn get (& self) -> i64 { unsafe { ffi :: ASN1_INTEGER_get (self . as_ptr ()) as i64 } } # [doc = " Converts the integer to a `BigNum`."] # [corresponds (ASN1_INTEGER_to_BN)] pub fn to_bn (& self) -> Result < BigNum , ErrorStack > { unsafe { cvt_p (ffi :: ASN1_INTEGER_to_BN (self . as_ptr () , ptr :: null_mut ())) . map (| p | BigNum :: from_ptr (p)) } } # [doc = " Sets the ASN.1 value to the value of a signed 32-bit integer, for larger numbers"] # [doc = " see [`bn`]."] # [doc = ""] # [doc = " [`bn`]: ../bn/struct.BigNumRef.html#method.to_asn1_integer"] # [corresponds (ASN1_INTEGER_set)] pub fn set (& mut self , value : i32) -> Result < () , ErrorStack > { unsafe { cvt (ffi :: ASN1_INTEGER_set (self . as_ptr () , value as c_long)) . map (| _ | ()) } } # [doc = " Creates a new Asn1Integer with the same value."] # [corresponds (ASN1_INTEGER_dup)] pub fn to_owned (& self) -> Result < Asn1Integer , ErrorStack > { unsafe { cvt_p (ffi :: ASN1_INTEGER_dup (self . as_ptr ())) . map (| p | Asn1Integer :: from_ptr (p)) } } }
};
}
