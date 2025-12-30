// Generated macro for try_parse_ref_op (function)
macro_rules! Depcrate_dereferencetry_parse_ref_op {
() => {
// Module: crate::dereference
// Provides: {"try_parse_ref_op"}
// Dependencies: {}
fn try_parse_ref_op < 'tcx > (tcx : TyCtxt < 'tcx > , typeck : & 'tcx TypeckResults < '_ > , expr : & 'tcx Expr < '_ > ,) -> Option < (RefOp , & 'tcx Expr < 'tcx > , Option < HirId >) > { let (call_path_id , def_id , arg) = match expr . kind { ExprKind :: MethodCall (_ , arg , [] , _) => (None , typeck . type_dependent_def_id (expr . hir_id) ? , arg) , ExprKind :: Call (& Expr { kind : ExprKind :: Path (QPath :: Resolved (None , path)) , hir_id , .. } , [arg] ,) => (Some (hir_id) , path . res . opt_def_id () ? , arg) , ExprKind :: Unary (UnOp :: Deref , sub_expr) if ! typeck . expr_ty (sub_expr) . is_raw_ptr () => { return Some ((RefOp :: Deref , sub_expr , None)) ; } , ExprKind :: AddrOf (BorrowKind :: Ref , mutability , sub_expr) => { return Some ((RefOp :: AddrOf (mutability) , sub_expr , None)) ; } , _ => return None , } ; let mutbl = match tcx . get_diagnostic_name (def_id) { Some (sym :: deref_method) => Mutability :: Not , Some (sym :: deref_mut_method) => Mutability :: Mut , _ => return None , } ; Some ((RefOp :: Method { mutbl , is_ufcs : call_path_id . is_some () , } , arg , call_path_id ,)) }
};
}
