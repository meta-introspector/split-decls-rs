// Generated macro for impl_1817 (impl)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperimpl_1817 {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"impl_1817"}
// Dependencies: {}
impl < S , F > FieldAliasMapper < S > for expression :: nullable :: Nullable < F > where F : FieldAliasMapper < S > , { type Out = expression :: nullable :: Nullable < < F as FieldAliasMapper < S > > :: Out > ; fn map (self , alias : & Alias < S >) -> Self :: Out { expression :: nullable :: Nullable :: new (self . 0 . map (alias)) } }
};
}
