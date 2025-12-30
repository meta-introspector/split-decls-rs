// Generated macro for impl_91 (impl)
macro_rules! Depcrate_resolver_errorsimpl_91 {
() => {
// Module: crate::resolver::errors
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > From < & InlineExpression < T > > for ResolverError where T : ToString , { fn from (exp : & InlineExpression < T >) -> Self { Self :: Reference (exp . into ()) } }
};
}
