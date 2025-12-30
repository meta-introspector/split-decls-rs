// Generated macro for get_types_from_cast (function)
macro_rules! Depcrate_checked_conversionsget_types_from_cast {
() => {
// Module: crate::checked_conversions
// Provides: {"get_types_from_cast"}
// Dependencies: {}
# [doc = " Tries to extract the from- and to-type from a cast expression"] fn get_types_from_cast (expr : & Expr < '_ > , types : & [Symbol] , func : Symbol , assoc_const : Symbol ,) -> Option < (Symbol , Symbol) > { let call_from_cast : Option < (& Expr < '_ > , Symbol) > = if let ExprKind :: Cast (limit , from_type) = & expr . kind && let TyKind :: Path (from_type_path) = & from_type . kind && let Some (from_sym) = int_ty_to_sym (from_type_path) { Some ((limit , from_sym)) } else { None } ; let limit_from : Option < (& Expr < '_ > , Symbol) > = call_from_cast . or_else (| | { if let ExprKind :: Call (from_func , [limit]) = & expr . kind && let ExprKind :: Path (path) = & from_func . kind && let Some (from_sym) = get_implementing_type (path , INTS , sym :: from) { Some ((limit , from_sym)) } else { None } }) ; if let Some ((limit , from_type)) = limit_from { match limit . kind { ExprKind :: Call (path , _) => { if let ExprKind :: Path (ref path) = path . kind && let Some (to_type) = get_implementing_type (path , types , func) { return Some ((from_type , to_type)) ; } } , ExprKind :: Path (ref path) => { if let Some (to_type) = get_implementing_type (path , types , assoc_const) { return Some ((from_type , to_type)) ; } } , _ => { } , } } None }
};
}
