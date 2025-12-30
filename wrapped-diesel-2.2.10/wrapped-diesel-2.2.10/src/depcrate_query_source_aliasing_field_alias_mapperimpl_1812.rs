// Generated macro for impl_1812 (impl)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperimpl_1812 {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"impl_1812"}
// Dependencies: {}
impl < S , C > FieldAliasMapper < S > for C where S : AliasSource , C : Column , S :: Target : FieldAliasMapperAssociatedTypesDisjointnessTrick < C :: Table , S , C > , { type Out = < S :: Target as FieldAliasMapperAssociatedTypesDisjointnessTrick < C :: Table , S , C > > :: Out ; fn map (self , alias : & Alias < S >) -> Self :: Out { < S :: Target as FieldAliasMapperAssociatedTypesDisjointnessTrick < C :: Table , S , C > > :: map (self , alias ,) } }
};
}
