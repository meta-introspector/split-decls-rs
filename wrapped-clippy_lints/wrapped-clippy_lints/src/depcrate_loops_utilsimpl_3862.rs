// Generated macro for impl_3862 (impl)
macro_rules! Depcrate_loops_utilsimpl_3862 {
() => {
// Module: crate::loops::utils
// Provides: {"impl_3862"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for IncrementVisitor < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { if let Some (def_id) = expr . res_local_id () { if let Some (parent) = get_parent_expr (self . cx , expr) { let state = self . states . entry (def_id) . or_insert (IncrementVisitorVarState :: Initial) ; if * state == IncrementVisitorVarState :: IncrOnce { * state = IncrementVisitorVarState :: DontWarn ; return ; } match parent . kind { ExprKind :: AssignOp (op , lhs , rhs) => { if lhs . hir_id == expr . hir_id { * state = if op . node == AssignOpKind :: AddAssign && is_integer_const (self . cx , rhs , 1) && * state == IncrementVisitorVarState :: Initial && self . depth == 0 { IncrementVisitorVarState :: IncrOnce } else { IncrementVisitorVarState :: DontWarn } ; } } , ExprKind :: Assign (lhs , _ , _) if lhs . hir_id == expr . hir_id => { * state = IncrementVisitorVarState :: DontWarn ; } , ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Mut , _) => { * state = IncrementVisitorVarState :: DontWarn ; } , _ => () , } } walk_expr (self , expr) ; } else if is_loop (expr) || is_conditional (expr) { self . depth += 1 ; walk_expr (self , expr) ; self . depth -= 1 ; } else if let ExprKind :: Continue (_) = expr . kind { self . depth += 1 ; } else { walk_expr (self , expr) ; } } }
};
}
