// Generated macro for PollReqContent (struct)
macro_rules! Depcrate_pollPollReqContent {
() => {
// Module: crate::poll
// Provides: {"PollReqContent"}
// Dependencies: {}
# [doc = " The `PollReqContent` type is defined in [RFC 4210 Section 5.3.22]."] # [doc = ""] # [doc = " ```text"] # [doc = "  PollReqContent ::= SEQUENCE OF SEQUENCE {"] # [doc = "      certReqId              INTEGER }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.22]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.22"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PollReqContent { pub cert_req_ids : Vec < PollReqContentId > , }
};
}
