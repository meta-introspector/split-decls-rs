// Generated macro for fn_sig_opt (function)
macro_rules! Depcrate_default_numeric_fallbackfn_sig_opt {
() => {
// Module: crate::default_numeric_fallback
// Provides: {"fn_sig_opt"}
// Dependencies: {}
fn fn_sig_opt < 'tcx > (cx : & LateContext < 'tcx > , hir_id : HirId) -> Option < PolyFnSig < 'tcx > > { let node_ty = cx . typeck_results () . node_type_opt (hir_id) ? ; match node_ty . kind () { ty :: FnDef (def_id , _) => Some (cx . tcx . fn_sig (* def_id) . instantiate_identity ()) , ty :: FnPtr (sig_tys , hdr) => Some (sig_tys . with (* hdr)) , _ => None , } }
};
}
