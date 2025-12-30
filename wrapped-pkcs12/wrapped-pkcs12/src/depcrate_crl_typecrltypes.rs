// Generated macro for CrlTypes (type)
macro_rules! Depcrate_crl_typeCrlTypes {
() => {
// Module: crate::crl_type
// Provides: {"CrlTypes"}
// Dependencies: {}
# [doc = " The `CRLTypes` type is defined in [RFC 7292 Section 4.2.4]."] # [doc = ""] # [doc = "```text"] # [doc = "  x509CRL BAG-TYPE ::="] # [doc = "      {OCTET STRING IDENTIFIED BY {crlTypes 1}}"] # [doc = "      -- DER-encoded X.509 CRL stored in OCTET STRING"] # [doc = ""] # [doc = "  CRLTypes BAG-TYPE ::= {"] # [doc = "      x509CRL,"] # [doc = "      ... -- For future extensions"] # [doc = "  }"] # [doc = "```"] # [doc = ""] # [doc = " [RFC 7292 Section 4.2.4]: https://www.rfc-editor.org/rfc/rfc7292#section-4.2.4"] pub type CrlTypes = OctetString ;
};
}
