// Generated macro for impl_1771 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1771 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1771"}
// Dependencies: {}
impl < S , C > QueryId for AliasedField < S , C > where S : AliasSource + 'static , S :: Target : 'static , C : Column < Table = S :: Target > + 'static + QueryId , { type QueryId = Self ; const HAS_STATIC_QUERY_ID : bool = < C as QueryId > :: HAS_STATIC_QUERY_ID ; }
};
}
