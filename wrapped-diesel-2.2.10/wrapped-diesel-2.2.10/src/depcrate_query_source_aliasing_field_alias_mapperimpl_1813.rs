// Generated macro for impl_1813 (impl)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperimpl_1813 {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"impl_1813"}
// Dependencies: {}
impl < TS , TC , S , C > FieldAliasMapperAssociatedTypesDisjointnessTrick < TC , S , C > for TS where S : AliasSource < Target = TS > , C : Column < Table = TC > , TC : Table , TS : TableNotEqual < TC > , { type Out = C ; fn map (column : C , _alias : & Alias < S >) -> Self :: Out { column } }
};
}
