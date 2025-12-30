// Generated macro for is_default_method_on_current_ty (function)
macro_rules! Depcrate_unconditional_recursionis_default_method_on_current_ty {
() => {
// Module: crate::unconditional_recursion
// Provides: {"is_default_method_on_current_ty"}
// Dependencies: {}
fn is_default_method_on_current_ty < 'tcx > (tcx : TyCtxt < 'tcx > , qpath : QPath < 'tcx > , implemented_ty_id : DefId) -> bool { match qpath { QPath :: Resolved (_ , path) => match path . segments { [first , .. , last] => last . ident . name == kw :: Default && first . res . opt_def_id () == Some (implemented_ty_id) , _ => false , } , QPath :: TypeRelative (ty , segment) => { if segment . ident . name != kw :: Default { return false ; } if matches ! (ty . kind , TyKind :: Path (QPath :: Resolved (_ , hir :: Path { res : Res :: SelfTyAlias { .. } , .. } ,))) { return true ; } get_hir_ty_def_id (tcx , * ty) == Some (implemented_ty_id) } , } }
};
}
