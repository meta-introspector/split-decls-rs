// Generated macro for default_where (function)
macro_rules! Depcrate_internals_genericsdefault_where {
() => {
// Module: crate::internals::generics
// Provides: {"default_where"}
// Dependencies: {}
pub fn default_where (where_clause : Option < & WhereClause >) -> WhereClause { where_clause . map_or_else (| | WhereClause { where_token : Default :: default () , predicates : Default :: default () , } , Clone :: clone ,) }
};
}
