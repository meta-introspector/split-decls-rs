// Generated macro for ensure_display_in_where_clause_for_type (function)
macro_rules! Depcrate_expandensure_display_in_where_clause_for_type {
() => {
// Module: crate::expand
// Provides: {"ensure_display_in_where_clause_for_type"}
// Dependencies: {}
# [doc = " Hygienically add `where _: Display` to the set of [TypeParamBound]s for `ident`, creating such"] # [doc = " a set if necessary."] fn ensure_display_in_where_clause_for_type (where_clause : & mut WhereClause , ident : Ident) { for pred_ty in where_clause . predicates . iter_mut () . flat_map (| predicate | match predicate { WherePredicate :: Type (pred_ty) => Some (pred_ty) , _ => None , }) { let matches_desired_type = matches ! (& pred_ty . bounded_ty , Type :: Path (TypePath { path , .. }) if Some (& ident) == path . get_ident ()) ; if matches_desired_type { add_display_constraint_to_type_predicate (pred_ty) ; return ; } } let mut new_type_predicate = new_empty_where_type_predicate (ident) ; add_display_constraint_to_type_predicate (& mut new_type_predicate) ; append_where_clause_type_predicate (where_clause , new_type_predicate) ; }
};
}
