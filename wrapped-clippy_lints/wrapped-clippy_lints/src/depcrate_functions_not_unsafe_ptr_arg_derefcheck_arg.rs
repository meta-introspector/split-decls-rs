// Generated macro for check_arg (function)
macro_rules! Depcrate_functions_not_unsafe_ptr_arg_derefcheck_arg {
() => {
// Module: crate::functions::not_unsafe_ptr_arg_deref
// Provides: {"check_arg"}
// Dependencies: {}
fn check_arg (cx : & LateContext < '_ > , raw_ptrs : & HirIdSet , arg : & hir :: Expr < '_ >) { if arg . res_local_id () . is_some_and (| id | raw_ptrs . contains (& id)) { span_lint (cx , NOT_UNSAFE_PTR_ARG_DEREF , arg . span , "this public function might dereference a raw pointer but is not marked `unsafe`" ,) ; } }
};
}
