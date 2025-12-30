// Generated macro for ErrorMsgContent (struct)
macro_rules! Depcrate_statusErrorMsgContent {
() => {
// Module: crate::status
// Provides: {"ErrorMsgContent"}
// Dependencies: {}
# [doc = " The `ErrorMsgContent` type is defined in [RFC 4210 Section 5.2.21]."] # [doc = ""] # [doc = " ```text"] # [doc = "  ErrorMsgContent ::= SEQUENCE {"] # [doc = "      pKIStatusInfo          PKIStatusInfo,"] # [doc = "      errorCode              INTEGER           OPTIONAL,"] # [doc = "      -- implementation-specific error codes"] # [doc = "      errorDetails           PKIFreeText       OPTIONAL"] # [doc = "      -- implementation-specific error details"] # [doc = "  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.21]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.21"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ErrorMsgContent < 'a > { pub pki_status_info : PkiStatusInfo < 'a > , pub error_code : Option < u64 > , pub error_details : Option < PkiFreeText < 'a > > , }
};
}
