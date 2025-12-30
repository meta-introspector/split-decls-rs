// Generated macro for PkiStatusInfo (struct)
macro_rules! Depcrate_statusPkiStatusInfo {
() => {
// Module: crate::status
// Provides: {"PkiStatusInfo"}
// Dependencies: {}
# [doc = " The `PKIStatusInfo` type is defined in [RFC 4210 Section 5.2.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "  PKIStatusInfo ::= SEQUENCE {"] # [doc = "      status        PKIStatus,"] # [doc = "      statusString  PKIFreeText     OPTIONAL,"] # [doc = "      failInfo      PKIFailureInfo  OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.3]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PkiStatusInfo < 'a > { pub status : PkiStatus , pub status_string : Option < PkiFreeText < 'a > > , pub fail_info : Option < PkiFailureInfo > , }
};
}
