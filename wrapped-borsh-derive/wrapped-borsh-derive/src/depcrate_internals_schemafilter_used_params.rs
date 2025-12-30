// Generated macro for filter_used_params (function)
macro_rules! Depcrate_internals_schemafilter_used_params {
() => {
// Module: crate::internals::schema
// Provides: {"filter_used_params"}
// Dependencies: {}
fn filter_used_params (generics : & Generics , not_skipped_type_params : HashSet < Ident >) -> Generics { let new_params = generics . params . clone () . into_iter () . filter (| param | match param { GenericParam :: Lifetime (..) | GenericParam :: Const (..) => true , GenericParam :: Type (ty_param) => not_skipped_type_params . contains (& ty_param . ident) , }) . collect () ; let mut where_clause = generics . where_clause . clone () ; where_clause = where_clause . map (| mut clause | { let new_predicates : Punctuated < WherePredicate , Comma > = clause . predicates . iter () . filter (| predicate | { # [cfg_attr (feature = "force_exhaustive_checks" , deny (non_exhaustive_omitted_patterns))] match predicate { WherePredicate :: Lifetime (..) => true , WherePredicate :: Type (predicate_type) => generics :: type_contains_some_param (& predicate_type . bounded_ty , & not_skipped_type_params ,) , _ => true , } }) . cloned () . collect () ; clause . predicates = new_predicates ; clause }) ; Generics { params : new_params , where_clause , .. generics . clone () } }
};
}
