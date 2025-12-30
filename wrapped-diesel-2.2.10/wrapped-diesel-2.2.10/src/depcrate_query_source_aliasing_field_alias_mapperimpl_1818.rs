// Generated macro for impl_1818 (impl)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperimpl_1818 {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"impl_1818"}
// Dependencies: {}
impl < S , F > FieldAliasMapper < S > for expression :: grouped :: Grouped < F > where F : FieldAliasMapper < S > , { type Out = expression :: grouped :: Grouped < < F as FieldAliasMapper < S > > :: Out > ; fn map (self , alias : & Alias < S >) -> Self :: Out { expression :: grouped :: Grouped (self . 0 . map (alias)) } }
};
}
