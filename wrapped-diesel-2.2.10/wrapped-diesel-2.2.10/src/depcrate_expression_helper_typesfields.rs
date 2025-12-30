// Generated macro for Fields (type)
macro_rules! Depcrate_expression_helper_typesFields {
() => {
// Module: crate::expression::helper_types
// Provides: {"Fields"}
// Dependencies: {}
# [doc = " The return type of [`alias.fields(fields)`](crate::query_source::Alias::fields)"] pub type Fields < Alias , Fields > = < Fields as crate :: query_source :: aliasing :: FieldAliasMapper < < Alias as crate :: query_source :: aliasing :: GetAliasSourceFromAlias > :: Source , > > :: Out ;
};
}
