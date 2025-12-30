// Generated macro for RevocationInfoChoices (struct)
macro_rules! Depcrate_revocationRevocationInfoChoices {
() => {
// Module: crate::revocation
// Provides: {"RevocationInfoChoices"}
// Dependencies: {}
# [doc = " The `RevocationInfoChoices` type is defined in [RFC 5652 Section 10.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RevocationInfoChoices ::= SET OF RevocationInfoChoice"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.1]: https://www.rfc-editor.org/rfc/rfc5652#section-10.2.1"] # [derive (Clone , Eq , PartialEq , Debug)] pub struct RevocationInfoChoices (pub SetOfVec < RevocationInfoChoice >) ;
};
}
