// Generated macro for impl_1059 (impl)
macro_rules! Depcrate_writebackimpl_1059 {
() => {
// Module: crate::writeback
// Provides: {"impl_1059"}
// Dependencies: {}
impl < 'cx , 'tcx > TypeFolder < TyCtxt < 'tcx > > for Resolver < 'cx , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . fcx . tcx } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { debug_assert ! (! r . is_bound () , "Should not be resolving bound region.") ; self . fcx . tcx . lifetimes . re_erased } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { self . handle_term (ty , Ty :: outer_exclusive_binder , Ty :: new_error) } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { self . handle_term (ct , ty :: Const :: outer_exclusive_binder , ty :: Const :: new_error) } fn fold_predicate (& mut self , predicate : ty :: Predicate < 'tcx >) -> ty :: Predicate < 'tcx > { assert ! (! self . should_normalize , "normalizing predicates in writeback is not generally sound") ; predicate . super_fold_with (self) } }
};
}
