// Generated macro for where_clause_or_default (function)
macro_rules! Depcrate_expandwhere_clause_or_default {
() => {
// Module: crate::expand
// Provides: {"where_clause_or_default"}
// Dependencies: {}
fn where_clause_or_default (clause : & mut Option < WhereClause >) -> & mut WhereClause { clause . get_or_insert_with (| | WhereClause { where_token : Default :: default () , predicates : Punctuated :: new () , }) }
};
}
