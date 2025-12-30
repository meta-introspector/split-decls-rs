// Generated macro for FieldAliasMapper (trait)
macro_rules! Depcrate_query_source_aliasing_field_alias_mapperFieldAliasMapper {
() => {
// Module: crate::query_source::aliasing::field_alias_mapper
// Provides: {"FieldAliasMapper"}
// Dependencies: {}
# [doc = " Serves to map `Self` to `Alias<S>`"] # [doc = ""] # [doc = " Any column `Self` that belongs to `S::Table` will be transformed into `AliasedField<S, Self>`"] # [doc = ""] # [doc = " Any column `Self` that does not belong to `S::Table` will be left untouched."] # [doc = ""] # [doc = " This also works with tuples and some expressions."] # [doc = ""] pub trait FieldAliasMapper < S > { # [doc = " Output type when mapping `C` to `Alias<S>`"] # [doc = ""] # [doc = " If `C: Column<Table = S::Table>`, `Out = AliasedField<S, C>`  "] # [doc = " Otherwise, `Out = C`"] type Out ; # [doc = " Does the mapping"] fn map (self , alias : & Alias < S >) -> Self :: Out ; }
};
}
