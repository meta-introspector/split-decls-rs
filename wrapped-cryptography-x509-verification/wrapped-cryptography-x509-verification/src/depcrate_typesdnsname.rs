// Generated macro for DNSName (struct)
macro_rules! Depcrate_typesDNSName {
() => {
// Module: crate::types
// Provides: {"DNSName"}
// Dependencies: {}
# [doc = " Represents a DNS name can be used in X.509 name matching."] # [doc = ""] # [doc = " A `DNSName` is an `asn1::IA5String` with additional invariant preservations"] # [doc = " per [RFC 5280 4.2.1.6], which in turn uses the preferred name syntax defined"] # [doc = " in [RFC 1034 3.5] and amended in [RFC 1123 2.1]."] # [doc = ""] # [doc = " Non-ASCII domain names (i.e., internationalized names) must be pre-encoded;"] # [doc = " comparisons are case-insensitive."] # [doc = ""] # [doc = " [RFC 5280 4.2.1.6]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.6"] # [doc = " [RFC 1034 3.5]: https://datatracker.ietf.org/doc/html/rfc1034#section-3.5"] # [doc = " [RFC 1123 2.1]: https://datatracker.ietf.org/doc/html/rfc1123#section-2.1"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use cryptography_x509_verification::types::DNSName;"] # [doc = " assert_eq!(DNSName::new(\"foo.com\").unwrap(), DNSName::new(\"FOO.com\").unwrap());"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct DNSName < 'a > (asn1 :: IA5String < 'a >) ;
};
}
