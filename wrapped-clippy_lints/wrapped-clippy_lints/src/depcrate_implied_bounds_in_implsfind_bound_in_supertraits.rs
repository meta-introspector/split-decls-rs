// Generated macro for find_bound_in_supertraits (function)
macro_rules! Depcrate_implied_bounds_in_implsfind_bound_in_supertraits {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"find_bound_in_supertraits"}
// Dependencies: {}
# [doc = " Given a bound in an `impl Trait` type, looks for a trait in the set of supertraits (previously"] # [doc = " collected in [`collect_supertrait_bounds`]) that matches (same trait and generic arguments)."] fn find_bound_in_supertraits < 'a , 'tcx > (cx : & LateContext < 'tcx > , trait_def_id : DefId , args : & 'tcx [GenericArg < 'tcx >] , bounds : & 'a [ImplTraitBound < 'tcx >] ,) -> Option < & 'a ImplTraitBound < 'tcx > > { bounds . iter () . find (| bound | { bound . predicates . iter () . any (| (clause , _) | { if let ClauseKind :: Trait (tr) = clause . kind () . skip_binder () && tr . def_id () == trait_def_id { is_same_generics (cx . tcx , tr . trait_ref . args , bound . args , args , bound . trait_def_id , trait_def_id ,) } else { false } }) }) }
};
}
