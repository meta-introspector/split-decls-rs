// Generated macro for arg_is_seek_from_current (function)
macro_rules! Depcrate_methods_seek_from_currentarg_is_seek_from_current {
() => {
// Module: crate::methods::seek_from_current
// Provides: {"arg_is_seek_from_current"}
// Dependencies: {}
fn arg_is_seek_from_current < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> bool { if let ExprKind :: Call (f , [arg]) = expr . kind && let ExprKind :: Path (ref path) = f . kind && let Some (ctor_call_id) = cx . qpath_res (path , f . hir_id) . opt_def_id () && is_enum_variant_ctor (cx , sym :: SeekFrom , sym :: Current , ctor_call_id) && let ExprKind :: Lit (lit) = arg . kind && let LitKind :: Int (Pu128 (0) , LitIntType :: Unsuffixed) = lit . node { return true ; } false }
};
}
