// Generated macro for PollRepContent (struct)
macro_rules! Depcrate_pollPollRepContent {
() => {
// Module: crate::poll
// Provides: {"PollRepContent"}
// Dependencies: {}
# [doc = " The `PollRepContent` type is defined in [RFC 4210 Section 5.3.22]."] # [doc = ""] # [doc = " ```text"] # [doc = "  PollRepContent ::= SEQUENCE OF SEQUENCE {"] # [doc = "      certReqId              INTEGER,"] # [doc = "      checkAfter             INTEGER,  -- time in seconds"] # [doc = "      reason                 PKIFreeText OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.22]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.22"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PollRepContent < 'a > { pub cert_req_id : PollReqContentId , pub check_after : u64 , pub reason : Option < PkiFreeText < 'a > > , }
};
}
