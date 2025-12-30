// Generated macro for internal (module)
macro_rules! Depcrate_matcher_support_auto_eqinternal {
() => {
// Module: crate::matcher_support::auto_eq
// Provides: {"internal"}
// Dependencies: {}
pub mod internal { use crate :: { matcher :: MatcherBase , matchers :: { eq , EqMatcher } , } ; pub struct Wrapper < T > (pub T) ; impl < T : MatcherBase > Wrapper < & '_ T > { # [inline] pub fn kind (& self) -> MatcherTag { MatcherTag } } pub trait ExpectedKind { # [inline] fn kind (& self) -> ExpectedTag { ExpectedTag } } impl < T > ExpectedKind for Wrapper < T > { } pub struct MatcherTag ; impl MatcherTag { # [inline] pub fn matcher < M > (self , matcher : M) -> M { matcher } } pub struct ExpectedTag ; impl ExpectedTag { # [inline] pub fn matcher < T > (self , expected : T) -> EqMatcher < T > { eq (expected) } } }
};
}
