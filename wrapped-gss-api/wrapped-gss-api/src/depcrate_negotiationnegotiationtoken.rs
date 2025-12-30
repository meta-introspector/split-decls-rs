// Generated macro for NegotiationToken (enum)
macro_rules! Depcrate_negotiationNegotiationToken {
() => {
// Module: crate::negotiation
// Provides: {"NegotiationToken"}
// Dependencies: {}
# [doc = " `NegotiationToken` as defined in [RFC 4178 Section 4.2]."] # [doc = ""] # [doc = " ```text"] # [doc = " NegotiationToken ::= CHOICE {"] # [doc = "     negTokenInit    [0] NegTokenInit,"] # [doc = "     negTokenResp    [1] NegTokenResp"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4178 Section 4.2]: https://datatracker.ietf.org/doc/html/rfc4178#section-4.2"] # [derive (Clone , Debug , PartialEq , Eq , Choice)] pub enum NegotiationToken < 'a > { # [doc = " This is the inner token of the initial negotiation message."] # [cfg (feature = "rfc2478")] # [asn1 (context_specific = "0" , constructed = "true" , tag_mode = "EXPLICIT")] NegTokenInit (NegTokenInit < 'a >) , # [doc = " The NegTokenInit2 message extends NegTokenInit with a negotiation hints (negHints) field."] # [cfg (not (feature = "rfc2478"))] # [asn1 (context_specific = "0" , constructed = "true" , tag_mode = "EXPLICIT")] NegTokenInit2 (NegTokenInit2 < 'a >) , # [doc = " Negotiation token returned by the target to the initiator which"] # [doc = " contains, for the first token returned, a global negotiation result"] # [doc = " and the security mechanism selected (if any)."] # [cfg (feature = "rfc2478")] # [asn1 (context_specific = "1" , constructed = "true" , tag_mode = "EXPLICIT")] NegTokenTarg (NegTokenTarg < 'a >) , # [doc = " This is the token for all subsequent negotiation messages."] # [cfg (not (feature = "rfc2478"))] # [asn1 (context_specific = "1" , constructed = "true" , tag_mode = "EXPLICIT")] NegTokenResp (NegTokenResp < 'a >) , }
};
}
