// Generated macro for impl_2657 (impl)
macro_rules! Depcrate_if_let_muteximpl_2657 {
() => {
// Module: crate::if_let_mutex
// Provides: {"impl_2657"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IfLetMutex { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if cx . tcx . sess . edition () >= Edition2024 { return ; } if let Some (higher :: IfLet { let_expr , if_then , if_else : Some (if_else) , .. }) = higher :: IfLet :: hir (cx , expr) && let Some (op_mutex) = for_each_expr_without_closures (let_expr , | e | mutex_lock_call (cx , e , None)) && let Some (arm_mutex) = for_each_expr_without_closures ((if_then , if_else) , | e | mutex_lock_call (cx , e , Some (op_mutex))) { let diag = | diag : & mut Diag < '_ , () > | { diag . span_label (op_mutex . span , "this Mutex will remain locked for the entire `if let`-block..." ,) ; diag . span_label (arm_mutex . span , "... and is tried to lock again here, which will always deadlock." ,) ; diag . help ("move the lock call outside of the `if let ...` expression") ; } ; span_lint_and_then (cx , IF_LET_MUTEX , expr . span , "calling `Mutex::lock` inside the scope of another `Mutex::lock` causes a deadlock" , diag ,) ; } } }
};
}
