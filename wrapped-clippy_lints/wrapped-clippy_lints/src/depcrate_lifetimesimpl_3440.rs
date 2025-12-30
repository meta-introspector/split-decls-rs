// Generated macro for impl_3440 (impl)
macro_rules! Depcrate_lifetimesimpl_3440 {
() => {
// Module: crate::lifetimes
// Provides: {"impl_3440"}
// Dependencies: {}
impl < 'cx , 'tcx , F > LifetimeChecker < 'cx , 'tcx , F > where F : NestedFilter < 'tcx > , { fn new (cx : & 'cx LateContext < 'tcx > , generics : & 'tcx Generics < '_ >) -> LifetimeChecker < 'cx , 'tcx , F > { let map = generics . params . iter () . filter_map (| par | match par . kind { GenericParamKind :: Lifetime { kind : LifetimeParamKind :: Explicit , } => Some ((par . def_id , Vec :: new ())) , _ => None , }) . collect () ; Self { cx , map , where_predicate_depth : 0 , bounded_ty_depth : 0 , generic_args_depth : 0 , lifetime_elision_impossible : false , phantom : std :: marker :: PhantomData , } } fn visit_where_bound_predicate (& mut self , hir_id : HirId , bounded_ty : & 'tcx Ty < 'tcx > , bounds : & 'tcx [GenericBound < 'tcx >] , bound_generic_params : & 'tcx [GenericParam < 'tcx >] ,) { try_visit ! (self . visit_id (hir_id)) ; self . bounded_ty_depth += 1 ; try_visit ! (self . visit_ty_unambig (bounded_ty)) ; self . bounded_ty_depth -= 1 ; walk_list ! (self , visit_param_bound , bounds) ; walk_list ! (self , visit_generic_param , bound_generic_params) ; } }
};
}
