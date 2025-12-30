// Generated macro for append_where_clause_type_predicate (function)
macro_rules! Depcrate_expandappend_where_clause_type_predicate {
() => {
// Module: crate::expand
// Provides: {"append_where_clause_type_predicate"}
// Dependencies: {}
# [doc = " Push `new_type_predicate` onto the end of `where_clause`."] fn append_where_clause_type_predicate (where_clause : & mut WhereClause , new_type_predicate : PredicateType ,) { if ! where_clause . predicates . is_empty () { where_clause . predicates . push_punct (Comma { spans : [Span :: call_site ()] , }) ; } where_clause . predicates . push_value (WherePredicate :: Type (new_type_predicate)) ; }
};
}
