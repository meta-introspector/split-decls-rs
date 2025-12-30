// Generated macro for is_self_sized (function)
macro_rules! Depcrate_parseis_self_sized {
() => {
// Module: crate::parse
// Provides: {"is_self_sized"}
// Dependencies: {}
fn is_self_sized (generics : & Generics) -> bool { if let Some (where_clause) = & generics . where_clause { for predicate in & where_clause . predicates { if let WherePredicate :: Type (pred_type) = predicate { if let Type :: Path (type_path) = & pred_type . bounded_ty { if type_path . path . is_ident ("Self") { for bound in & pred_type . bounds { if let TypeParamBound :: Trait (trait_bound) = bound { if trait_bound . path . is_ident ("Sized") { return true ; } } } } } } } } false }
};
}
