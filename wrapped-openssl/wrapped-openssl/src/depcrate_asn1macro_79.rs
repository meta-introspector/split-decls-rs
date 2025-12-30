// Generated macro for macro_79 (macro)
macro_rules! Depcrate_asn1macro_79 {
() => {
// Module: crate::asn1
// Provides: {"macro_79"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: ASN1_GENERALIZEDTIME ; fn drop = ffi :: ASN1_GENERALIZEDTIME_free ; # [doc = " Non-UTC representation of time"] # [doc = ""] # [doc = " If a time can be represented by UTCTime, UTCTime is used"] # [doc = " otherwise, ASN1_GENERALIZEDTIME is used.  This would be, for"] # [doc = " example outside the year range of 1950-2049."] # [doc = ""] # [doc = " [ASN1_GENERALIZEDTIME_set] documentation from OpenSSL provides"] # [doc = " further details of implementation.  Note: these docs are from the master"] # [doc = " branch as documentation on the 1.1.0 branch did not include this page."] # [doc = ""] # [doc = " [ASN1_GENERALIZEDTIME_set]: https://docs.openssl.org/master/man3/ASN1_GENERALIZEDTIME_set/"] pub struct Asn1GeneralizedTime ; # [doc = " Reference to a [`Asn1GeneralizedTime`]"] # [doc = ""] # [doc = " [`Asn1GeneralizedTime`]: struct.Asn1GeneralizedTime.html"] pub struct Asn1GeneralizedTimeRef ; }
};
}
