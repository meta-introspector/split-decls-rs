// Generated macro for Generics (struct)
macro_rules! Depcrate_ast_genericsGenerics {
() => {
// Module: crate::ast::generics
// Provides: {"Generics"}
// Dependencies: {}
# [doc = " A mirror of the `syn::Generics` type which can contain arbitrary representations"] # [doc = " of params and where clauses."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Generics < P , W = syn :: WhereClause > { pub params : Vec < P > , pub where_clause : Option < W > , }
};
}
