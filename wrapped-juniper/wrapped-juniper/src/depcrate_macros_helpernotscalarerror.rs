// Generated macro for NotScalarError (struct)
macro_rules! Depcrate_macros_helperNotScalarError {
() => {
// Module: crate::macros::helper
// Provides: {"NotScalarError"}
// Dependencies: {}
# [doc = " Error of an [`InputValue`] not representing a [`ScalarValue`], used in macro expansions."] # [derive (Display)] # [display ("Expected GraphQL scalar, found: {_0}")] pub struct NotScalarError < 'a , S : ScalarValue > (pub & 'a InputValue < S >) ;
};
}
