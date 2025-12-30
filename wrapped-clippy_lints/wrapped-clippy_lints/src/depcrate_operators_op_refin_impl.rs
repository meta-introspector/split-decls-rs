// Generated macro for in_impl (function)
macro_rules! Depcrate_operators_op_refin_impl {
() => {
// Module: crate::operators::op_ref
// Provides: {"in_impl"}
// Dependencies: {}
fn in_impl < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , bin_op : DefId ,) -> Option < (& 'tcx rustc_hir :: Ty < 'tcx > , & 'tcx rustc_hir :: Ty < 'tcx >) > { if let Some (block) = get_enclosing_block (cx , e . hir_id) && let Some (impl_def_id) = cx . tcx . impl_of_assoc (block . hir_id . owner . to_def_id ()) && let item = cx . tcx . hir_expect_item (impl_def_id . expect_local ()) && let ItemKind :: Impl (item) = & item . kind && let Some (of_trait) = & item . of_trait && let Some (seg) = of_trait . trait_ref . path . segments . last () && let Res :: Def (_ , trait_id) = seg . res && trait_id == bin_op && let Some (generic_args) = seg . args && let Some (GenericArg :: Type (other_ty)) = generic_args . args . last () { Some ((item . self_ty , other_ty . as_unambig_ty ())) } else { None } }
};
}
