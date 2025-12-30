// Generated macro for where_clause_with_bound (function)
macro_rules! Depcrate_boundwhere_clause_with_bound {
() => {
// Module: crate::bound
// Provides: {"where_clause_with_bound"}
// Dependencies: {}
pub (crate) fn where_clause_with_bound (generics : & Generics , bound : TokenStream) -> WhereClause { let new_predicates = generics . type_params () . map :: < WherePredicate , _ > (| param | { let param = & param . ident ; parse_quote ! (# param : # bound) }) ; let mut generics = generics . clone () ; generics . make_where_clause () . predicates . extend (new_predicates) ; generics . where_clause . unwrap () }
};
}
