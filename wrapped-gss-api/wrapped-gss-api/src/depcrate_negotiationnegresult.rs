// Generated macro for NegResult (enum)
macro_rules! Depcrate_negotiationNegResult {
() => {
// Module: crate::negotiation
// Provides: {"NegResult"}
// Dependencies: {}
# [doc = " `NegResult` as defined in [RFC 2479 Section 3.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " NegTokenTarg ::= SEQUENCE {"] # [doc = "     negResult      [0] ENUMERATED {"] # [doc = "                             accept_completed    (0),"] # [doc = "                             accept_incomplete   (1),"] # [doc = "                             reject              (2) }          OPTIONAL,"] # [doc = "     supportedMech  [1] MechType                                OPTIONAL,"] # [doc = "     responseToken  [2] OCTET STRING                            OPTIONAL,"] # [doc = "     mechListMIC    [3] OCTET STRING                            OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 2479 Section 3.2.1]: https://datatracker.ietf.org/doc/html/rfc2478#section-3.2.1"] # [cfg (feature = "rfc2478")] # [derive (Clone , Debug , Copy , PartialEq , Eq , PartialOrd , Ord , Enumerated)] # [asn1 (type = "ENUMERATED")] # [repr (u8)] # [allow (missing_docs)] pub enum NegResult { # [doc = " The target accepts the preferred security mechanism, and the context is established for the target."] AcceptCompleted = 0 , # [doc = " The target accepts one of the proposed security mechanisms and further exchanges are necessary."] AcceptIncomplete = 1 , # [doc = " The target rejects all the proposed security mechanisms."] Reject = 2 , }
};
}
