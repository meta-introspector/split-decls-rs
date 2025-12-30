// Generated macro for has_disaplayable_predicates (function)
macro_rules! Depcrate_displayhas_disaplayable_predicates {
() => {
// Module: crate::display
// Provides: {"has_disaplayable_predicates"}
// Dependencies: {}
fn has_disaplayable_predicates (db : & dyn HirDatabase , params : & GenericParams , store : & ExpressionStore ,) -> bool { params . where_predicates () . iter () . any (| pred | { ! matches ! (pred , WherePredicate :: TypeBound { target , .. } if matches ! (store [* target] , TypeRef :: TypeParam (id) if db . generic_params (id . parent ()) [id . local_id ()] . name () . is_none ())) }) }
};
}
