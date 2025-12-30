// Generated macro for new_empty_where_clause (function)
macro_rules! Depcrate_expandnew_empty_where_clause {
() => {
// Module: crate::expand
// Provides: {"new_empty_where_clause"}
// Dependencies: {}
# [doc = " Create a `where` clause that we can add [WherePredicate]s to."] fn new_empty_where_clause () -> WhereClause { WhereClause { where_token : Where { span : Span :: call_site () , } , predicates : Punctuated :: < WherePredicate , Comma > :: new () , } }
};
}
