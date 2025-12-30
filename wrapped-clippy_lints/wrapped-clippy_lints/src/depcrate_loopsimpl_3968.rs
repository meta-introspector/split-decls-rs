// Generated macro for impl_3968 (impl)
macro_rules! Depcrate_loopsimpl_3968 {
() => {
// Module: crate::loops
// Provides: {"impl_3968"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Loops { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let for_loop = higher :: ForLoop :: hir (expr) ; if let Some (higher :: ForLoop { pat , arg , body , loop_id , span , label , }) = for_loop { if body . span . from_expansion () { return ; } self . check_for_loop (cx , pat , arg , body , expr , span , label) ; if let ExprKind :: Block (block , _) = body . kind { never_loop :: check (cx , block , loop_id , span , for_loop . as_ref ()) ; } } if expr . span . from_expansion () { return ; } if let ExprKind :: Loop (block , ..) = expr . kind { never_loop :: check (cx , block , expr . hir_id , expr . span , None) ; } if let ExprKind :: Loop (block , label , LoopSource :: Loop , _) = expr . kind { empty_loop :: check (cx , expr , block) ; while_let_loop :: check (cx , expr , block) ; infinite_loop :: check (cx , expr , block , label) ; } while_let_on_iterator :: check (cx , expr) ; if let Some (higher :: While { condition , body , span , .. }) = higher :: While :: hir (expr) { while_immutable_condition :: check (cx , condition , body) ; while_float :: check (cx , condition) ; missing_spin_loop :: check (cx , condition , body) ; manual_while_let_some :: check (cx , condition , body , span) ; } if let ExprKind :: MethodCall (path , recv , [arg] , _) = expr . kind && matches ! (path . ident . name , sym :: all | sym :: any | sym :: filter_map | sym :: find_map | sym :: flat_map | sym :: for_each | sym :: map) { unused_enumerate_index :: check_method (cx , expr , recv , arg) ; } } }
};
}
