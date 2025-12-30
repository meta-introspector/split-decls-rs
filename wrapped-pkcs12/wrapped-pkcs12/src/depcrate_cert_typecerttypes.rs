// Generated macro for CertTypes (type)
macro_rules! Depcrate_cert_typeCertTypes {
() => {
// Module: crate::cert_type
// Provides: {"CertTypes"}
// Dependencies: {}
# [doc = " The `CertTypes` type is defined in [RFC 7292 Section 4.2.3]."] # [doc = ""] # [doc = "```text"] # [doc = "    x509Certificate BAG-TYPE ::="] # [doc = "        {OCTET STRING IDENTIFIED BY {certTypes 1}}"] # [doc = "        -- DER-encoded X.509 certificate stored in OCTET STRING"] # [doc = "    sdsiCertificate BAG-TYPE ::="] # [doc = "        {IA5String IDENTIFIED BY {certTypes 2}}"] # [doc = "        -- Base64-encoded SDSI certificate stored in IA5String"] # [doc = ""] # [doc = "    CertTypes BAG-TYPE ::= {"] # [doc = "        x509Certificate |"] # [doc = "        sdsiCertificate,"] # [doc = "        ... -- For future extensions"] # [doc = "    }"] # [doc = "```"] # [doc = ""] # [doc = " [RFC 7292 Section 4.2.3]: https://www.rfc-editor.org/rfc/rfc7292#section-4.2.3"] pub type CertTypes = OctetString ;
};
}
