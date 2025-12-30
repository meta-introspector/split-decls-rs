// Generated macro for impl_81 (impl)
macro_rules! Depcrate_asn1impl_81 {
() => {
// Module: crate::asn1
// Provides: {"impl_81"}
// Dependencies: {}
impl Asn1GeneralizedTime { # [doc = " Creates a new generalized time corresponding to the specified ASN1 time"] # [doc = " string."] # [corresponds (ASN1_GENERALIZEDTIME_set_string)] # [allow (clippy :: should_implement_trait)] pub fn from_str (s : & str) -> Result < Asn1GeneralizedTime , ErrorStack > { unsafe { ffi :: init () ; let time_str = CString :: new (s) . unwrap () ; let ptr = cvt_p (ffi :: ASN1_GENERALIZEDTIME_new ()) ? ; let time = Asn1GeneralizedTime :: from_ptr (ptr) ; cvt (ffi :: ASN1_GENERALIZEDTIME_set_string (time . as_ptr () , time_str . as_ptr () ,)) ? ; Ok (time) } } }
};
}
