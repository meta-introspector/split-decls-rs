// Generated macro for get_hir_ty_def_id (function)
macro_rules! Depcrate_unconditional_recursionget_hir_ty_def_id {
() => {
// Module: crate::unconditional_recursion
// Provides: {"get_hir_ty_def_id"}
// Dependencies: {}
fn get_hir_ty_def_id < 'tcx > (tcx : TyCtxt < 'tcx > , hir_ty : rustc_hir :: Ty < 'tcx >) -> Option < DefId > { let TyKind :: Path (qpath) = hir_ty . kind else { return None } ; match qpath { QPath :: Resolved (_ , path) => path . res . opt_def_id () , QPath :: TypeRelative (_ , _) => { let ty = lower_ty (tcx , & hir_ty) ; match ty . kind () { ty :: Alias (ty :: Projection , proj) => { Res :: < HirId > :: Def (DefKind :: Trait , proj . trait_ref (tcx) . def_id) . opt_def_id () } , _ => None , } } , } }
};
}
