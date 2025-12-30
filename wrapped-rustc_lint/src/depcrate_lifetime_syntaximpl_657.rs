// Generated macro for impl_657 (impl)
macro_rules! Depcrate_lifetime_syntaximpl_657 {
() => {
// Module: crate::lifetime_syntax
// Provides: {"impl_657"}
// Dependencies: {}
impl < 'a , 'tcx > LifetimeInfoCollector < 'a , 'tcx > { fn collect (ty : & 'tcx hir :: Ty < 'tcx > , map : & 'a mut LifetimeInfoMap < 'tcx >) { let mut this = Self { type_span : ty . span , referenced_type_span : None , map } ; intravisit :: walk_unambig_ty (& mut this , ty) ; } }
};
}
