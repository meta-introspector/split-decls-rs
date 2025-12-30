// Generated macro for add_constraints_from_crate (function)
macro_rules! Depcrate_variance_constraintsadd_constraints_from_crate {
() => {
// Module: crate::variance::constraints
// Provides: {"add_constraints_from_crate"}
// Dependencies: {}
pub (crate) fn add_constraints_from_crate < 'a , 'tcx > (terms_cx : TermsContext < 'a , 'tcx > ,) -> ConstraintContext < 'a , 'tcx > { let tcx = terms_cx . tcx ; let covariant = terms_cx . arena . alloc (ConstantTerm (ty :: Covariant)) ; let contravariant = terms_cx . arena . alloc (ConstantTerm (ty :: Contravariant)) ; let invariant = terms_cx . arena . alloc (ConstantTerm (ty :: Invariant)) ; let bivariant = terms_cx . arena . alloc (ConstantTerm (ty :: Bivariant)) ; let mut constraint_cx = ConstraintContext { terms_cx , covariant , contravariant , invariant , bivariant , constraints : Vec :: new () , } ; let crate_items = tcx . hir_crate_items (()) ; for def_id in crate_items . definitions () { let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: Struct | DefKind :: Union | DefKind :: Enum => { constraint_cx . build_constraints_for_item (def_id) ; let adt = tcx . adt_def (def_id) ; for variant in adt . variants () { if let Some (ctor_def_id) = variant . ctor_def_id () { constraint_cx . build_constraints_for_item (ctor_def_id . expect_local ()) ; } } } DefKind :: Fn | DefKind :: AssocFn => constraint_cx . build_constraints_for_item (def_id) , DefKind :: TyAlias if tcx . type_alias_is_lazy (def_id) => { constraint_cx . build_constraints_for_item (def_id) } _ => { } } } constraint_cx }
};
}
