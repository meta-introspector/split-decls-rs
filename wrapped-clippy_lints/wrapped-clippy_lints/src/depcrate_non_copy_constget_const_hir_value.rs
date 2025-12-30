// Generated macro for get_const_hir_value (function)
macro_rules! Depcrate_non_copy_constget_const_hir_value {
() => {
// Module: crate::non_copy_const
// Provides: {"get_const_hir_value"}
// Dependencies: {}
# [doc = " Attempts to get the value of a constant as a HIR expression. Also gets the"] # [doc = " `TypeckResults` associated with the constant's body."] fn get_const_hir_value < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , did : DefId , args : GenericArgsRef < 'tcx > ,) -> Option < (& 'tcx TypeckResults < 'tcx > , & 'tcx Expr < 'tcx >) > { let did = did . as_local () ? ; let (did , ct_rhs) = match tcx . hir_node (tcx . local_def_id_to_hir_id (did)) { Node :: Item (item) if let ItemKind :: Const (.. , ct_rhs) = item . kind => (did , ct_rhs) , Node :: ImplItem (item) if let ImplItemKind :: Const (.. , ct_rhs) = item . kind => (did , ct_rhs) , Node :: TraitItem (_) if let Ok (Some (inst)) = Instance :: try_resolve (tcx , typing_env , did . into () , args) && let Some (did) = inst . def_id () . as_local () => { match tcx . hir_node (tcx . local_def_id_to_hir_id (did)) { Node :: ImplItem (item) if let ImplItemKind :: Const (.. , ct_rhs) = item . kind => (did , ct_rhs) , Node :: TraitItem (item) if let TraitItemKind :: Const (.. , Some (ct_rhs)) = item . kind => (did , ct_rhs) , _ => return None , } } , _ => return None , } ; match ct_rhs { ConstItemRhs :: Body (body_id) => Some ((tcx . typeck (did) , tcx . hir_body (body_id) . value)) , ConstItemRhs :: TypeConst (ct_arg) => match ct_arg . kind { ConstArgKind :: Anon (anon_const) => Some ((tcx . typeck (did) , tcx . hir_body (anon_const . body) . value)) , _ => None , } , } }
};
}
