// Generated macro for FieldAliasMapperAssociatedTypesDisjointnessTrick (trait)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperFieldAliasMapperAssociatedTypesDisjointnessTrick {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"FieldAliasMapperAssociatedTypesDisjointnessTrick"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Allows implementing `FieldAliasMapper` in external crates without running into conflicting impl"] # [doc = " errors due to https://github.com/rust-lang/rust/issues/20400"] # [doc = ""] # [doc = " We will always have `Self = S::Table` and `CT = C::Table`"] pub trait FieldAliasMapperAssociatedTypesDisjointnessTrick < CT , S , C > { type Out ; fn map (column : C , alias : & Alias < S >) -> Self :: Out ; }
};
}
