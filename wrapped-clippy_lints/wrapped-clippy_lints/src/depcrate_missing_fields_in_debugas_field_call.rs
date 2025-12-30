// Generated macro for as_field_call (function)
macro_rules! Depcrate_missing_fields_in_debugas_field_call {
() => {
// Module: crate::missing_fields_in_debug
// Provides: {"as_field_call"}
// Dependencies: {}
# [doc = " Checks if the given expression is a call to `DebugStruct::field`"] # [doc = " and the first argument to it is a string literal and if so, returns it"] # [doc = ""] # [doc = " Example: `.field(\"foo\", ....)` returns `Some(\"foo\")`"] fn as_field_call < 'tcx > (cx : & LateContext < 'tcx > , typeck_results : & TypeckResults < 'tcx > , expr : & Expr < '_ > ,) -> Option < Symbol > { if let ExprKind :: MethodCall (path , recv , [debug_field , _] , _) = & expr . kind && let recv_ty = typeck_results . expr_ty (recv) . peel_refs () && recv_ty . is_diag_item (cx , sym :: DebugStruct) && path . ident . name == sym :: field && let ExprKind :: Lit (lit) = & debug_field . kind && let LitKind :: Str (sym , ..) = lit . node { Some (sym) } else { None } }
};
}
