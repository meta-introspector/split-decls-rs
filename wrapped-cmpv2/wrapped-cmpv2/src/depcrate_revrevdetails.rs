// Generated macro for RevDetails (struct)
macro_rules! Depcrate_revRevDetails {
() => {
// Module: crate::rev
// Provides: {"RevDetails"}
// Dependencies: {}
# [doc = " The `RevDetails` type is defined in [RFC 4210 Section 5.3.9]."] # [doc = ""] # [doc = " ```text"] # [doc = "  RevDetails ::= SEQUENCE {"] # [doc = "      certDetails         CertTemplate,"] # [doc = "      -- allows requester to specify as much as they can about"] # [doc = "      -- the cert. for which revocation is requested"] # [doc = "      -- (e.g., for cases in which serialNumber is not available)"] # [doc = "      crlEntryDetails     Extensions{{...}}    OPTIONAL"] # [doc = "      -- requested crlEntryExtensions"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.9]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.9"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct RevDetails { pub cert_details : CertTemplate , pub crl_entry_details : Option < Extensions > , }
};
}
