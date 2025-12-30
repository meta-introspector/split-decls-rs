// Generated macro for impl_199 (impl)
macro_rules! Depcrate_either_or_bothimpl_199 {
() => {
// Module: crate::either_or_both
// Provides: {"impl_199"}
// Dependencies: {}
impl < A , B > From < Either < A , B > > for EitherOrBoth < A , B > { fn from (either : Either < A , B >) -> Self { match either { Either :: Left (l) => Left (l) , Either :: Right (l) => Right (l) , } } }
};
}
