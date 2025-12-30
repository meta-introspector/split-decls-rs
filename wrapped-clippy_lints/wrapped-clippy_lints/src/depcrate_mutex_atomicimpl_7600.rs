// Generated macro for impl_7600 (impl)
macro_rules! Depcrate_mutex_atomicimpl_7600 {
() => {
// Module: crate::mutex_atomic
// Provides: {"impl_7600"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Mutex { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let ty = cx . typeck_results () . expr_ty (expr) ; if let ty :: Adt (_ , subst) = ty . kind () && is_type_diagnostic_item (cx , ty , sym :: Mutex) { let mutex_param = subst . type_at (0) ; if let Some (atomic_name) = get_atomic_name (mutex_param) { let msg = format ! ("consider using an `{atomic_name}` instead of a `Mutex` here; if you just want the locking \
                         behavior and not the internal type, consider using `Mutex<()>`") ; match * mutex_param . kind () { ty :: Uint (t) if t != UintTy :: Usize => span_lint (cx , MUTEX_INTEGER , expr . span , msg) , ty :: Int (t) if t != IntTy :: Isize => span_lint (cx , MUTEX_INTEGER , expr . span , msg) , _ => span_lint (cx , MUTEX_ATOMIC , expr . span , msg) , } } } } }
};
}
