// Generated macro for const_item_rhs_to_expr (function)
macro_rules! Depcrate_constsconst_item_rhs_to_expr {
() => {
// Module: crate::consts
// Provides: {"const_item_rhs_to_expr"}
// Dependencies: {}
pub fn const_item_rhs_to_expr < 'tcx > (tcx : TyCtxt < 'tcx > , ct_rhs : ConstItemRhs < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { match ct_rhs { ConstItemRhs :: Body (body_id) => Some (tcx . hir_body (body_id) . value) , ConstItemRhs :: TypeConst (const_arg) => match const_arg . kind { ConstArgKind :: Anon (anon) => Some (tcx . hir_body (anon . body) . value) , ConstArgKind :: Path (_) | ConstArgKind :: Error (..) | ConstArgKind :: Infer (..) => None , } , } }
};
}
