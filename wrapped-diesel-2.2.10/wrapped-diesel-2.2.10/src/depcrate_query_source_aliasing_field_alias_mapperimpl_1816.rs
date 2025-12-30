// Generated macro for impl_1816 (impl)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperimpl_1816 {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"impl_1816"}
// Dependencies: {}
impl < SPrev , SNew , F > FieldAliasMapper < SNew > for AliasedField < SPrev , F > where SNew : AliasSource , { type Out = Self ; fn map (self , _alias : & Alias < SNew >) -> Self :: Out { self } }
};
}
