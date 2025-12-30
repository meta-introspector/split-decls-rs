// Generated macro for impl_198 (impl)
macro_rules! Depcrate_either_or_bothimpl_198 {
() => {
// Module: crate::either_or_both
// Provides: {"impl_198"}
// Dependencies: {}
impl < A , B > From < EitherOrBoth < A , B > > for Option < Either < A , B > > { fn from (value : EitherOrBoth < A , B >) -> Self { match value { Left (l) => Some (Either :: Left (l)) , Right (r) => Some (Either :: Right (r)) , Both (..) => None , } } }
};
}
