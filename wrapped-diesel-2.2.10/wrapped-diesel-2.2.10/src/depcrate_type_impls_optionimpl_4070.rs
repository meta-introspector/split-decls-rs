// Generated macro for impl_4070 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4070 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4070"}
// Dependencies: {}
impl < T > QueryId for Nullable < T > where T : QueryId + SqlType < IsNull = is_nullable :: NotNull > , { type QueryId = T :: QueryId ; const HAS_STATIC_QUERY_ID : bool = T :: HAS_STATIC_QUERY_ID ; }
};
}
