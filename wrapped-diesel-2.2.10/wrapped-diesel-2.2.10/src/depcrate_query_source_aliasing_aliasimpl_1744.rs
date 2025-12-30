// Generated macro for impl_1744 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1744 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1744"}
// Dependencies: {}
impl < S > QueryId for Alias < S > where Self : 'static , S : AliasSource , S :: Target : QueryId , { type QueryId = Self ; const HAS_STATIC_QUERY_ID : bool = < S :: Target as QueryId > :: HAS_STATIC_QUERY_ID ; }
};
}
