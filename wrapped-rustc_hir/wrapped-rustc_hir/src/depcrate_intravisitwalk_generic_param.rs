// Generated macro for walk_generic_param (function)
macro_rules! Depcrate_intravisitwalk_generic_param {
() => {
// Module: crate::intravisit
// Provides: {"walk_generic_param"}
// Dependencies: {}
pub fn walk_generic_param < 'v , V : Visitor < 'v > > (visitor : & mut V , param : & 'v GenericParam < 'v > ,) -> V :: Result { let GenericParam { hir_id , def_id : _ , name , span : _ , pure_wrt_drop : _ , kind , colon_span : _ , source : _ , } = param ; try_visit ! (visitor . visit_id (* hir_id)) ; match * name { ParamName :: Plain (ident) | ParamName :: Error (ident) => try_visit ! (visitor . visit_ident (ident)) , ParamName :: Fresh => { } } match * kind { GenericParamKind :: Lifetime { .. } => { } GenericParamKind :: Type { ref default , .. } => { visit_opt ! (visitor , visit_ty_unambig , default) } GenericParamKind :: Const { ref ty , ref default } => { try_visit ! (visitor . visit_ty_unambig (ty)) ; if let Some (default) = default { try_visit ! (visitor . visit_const_param_default (* hir_id , default)) ; } } } V :: Result :: output () }
};
}
