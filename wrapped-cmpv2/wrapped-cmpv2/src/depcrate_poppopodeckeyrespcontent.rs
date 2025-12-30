// Generated macro for PopoDecKeyRespContent (type)
macro_rules! Depcrate_popPopoDecKeyRespContent {
() => {
// Module: crate::pop
// Provides: {"PopoDecKeyRespContent"}
// Dependencies: {}
# [doc = " The `POPODecKeyRespContent` type is defined in [RFC 4210 Section 5.2.8.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "  POPODecKeyRespContent ::= SEQUENCE OF INTEGER"] # [doc = "  -- One INTEGER per encryption key certification request (in the"] # [doc = "  -- same order as these requests appear in CertReqMessages).  The"] # [doc = "  -- retrieved INTEGER A (above) is returned to the sender of the"] # [doc = "  -- corresponding Challenge."] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.2.8.3]: https://www.rfc-editor.org/rfc/rfc4210#section-5.2.8.3"] pub type PopoDecKeyRespContent < 'a > = Vec < UintRef < 'a > > ;
};
}
