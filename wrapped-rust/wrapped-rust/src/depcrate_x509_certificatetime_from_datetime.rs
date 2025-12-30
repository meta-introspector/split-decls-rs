// Generated macro for time_from_datetime (function)
macro_rules! Depcrate_x509_certificatetime_from_datetime {
() => {
// Module: crate::x509::certificate
// Provides: {"time_from_datetime"}
// Dependencies: {}
pub (crate) fn time_from_datetime (dt : asn1 :: DateTime) -> CryptographyResult < common :: Time > { if dt . year () >= 2050 { Ok (common :: Time :: GeneralizedTime (asn1 :: X509GeneralizedTime :: new (dt) ? ,)) } else { Ok (common :: Time :: UtcTime (asn1 :: UtcTime :: new (dt) . unwrap ())) } }
};
}
