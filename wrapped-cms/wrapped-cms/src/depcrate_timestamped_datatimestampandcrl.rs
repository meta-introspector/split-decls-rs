// Generated macro for TimeStampAndCrl (struct)
macro_rules! Depcrate_timestamped_dataTimeStampAndCrl {
() => {
// Module: crate::timestamped_data
// Provides: {"TimeStampAndCrl"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " TimeStampAndCrl ::= SEQUENCE {"] # [doc = "     timeStamp   TimeStampToken,          -- according to RFC 3161"] # [doc = "     crl         CertificateList OPTIONAL -- according to RFC 5280"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct TimeStampAndCrl { pub time_stamp : TimeStampToken , # [asn1 (optional = "true")] pub crl : Option < CertificateList > , }
};
}
