// Generated macro for ensure_where_clause_has_display_for_all_unconstrained_members (function)
macro_rules! Depcrate_expandensure_where_clause_has_display_for_all_unconstrained_members {
() => {
// Module: crate::expand
// Provides: {"ensure_where_clause_has_display_for_all_unconstrained_members"}
// Dependencies: {}
# [doc = " For all declared type parameters, add a [core::fmt::Display] constraint, unless the type"] # [doc = " parameter already has any type constraint."] fn ensure_where_clause_has_display_for_all_unconstrained_members (where_clause : & mut WhereClause , type_params : & [& TypeParam] ,) { let param_constraint_mapping = extract_trait_constraints_from_source (where_clause , type_params) ; for (ident , known_bounds) in param_constraint_mapping . into_iter () { if known_bounds . is_empty () { ensure_display_in_where_clause_for_type (where_clause , ident) ; } } }
};
}
