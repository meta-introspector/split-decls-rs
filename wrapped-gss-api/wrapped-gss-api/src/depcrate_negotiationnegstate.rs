// Generated macro for NegState (enum)
macro_rules! Depcrate_negotiationNegState {
() => {
// Module: crate::negotiation
// Provides: {"NegState"}
// Dependencies: {}
# [derive (Clone , Debug , Copy , PartialEq , Eq , PartialOrd , Ord , Enumerated)] # [asn1 (type = "ENUMERATED")] # [repr (u8)] # [allow (missing_docs)] pub enum NegState { # [doc = " No further negotiation message from the peer is expected, and"] # [doc = " the security context is established for the sender."] AcceptCompleted = 0 , # [doc = " At least one additional negotiation message from the peer is"] # [doc = " needed to establish the security context."] AcceptIncomplete = 1 , # [doc = " The sender terminates the negotiation."] Reject = 2 , # [doc = " The sender indicates that the exchange of MIC tokens, as"] # [doc = " described in Section 5, will be REQUIRED if per-message"] # [doc = " integrity services are available on the mechanism context to be"] # [doc = " established.  This value SHALL only be present in the first"] # [doc = " reply from the target."] RequestMic = 3 , }
};
}
