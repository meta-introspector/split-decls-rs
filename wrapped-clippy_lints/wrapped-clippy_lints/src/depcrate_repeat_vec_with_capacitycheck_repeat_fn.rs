// Generated macro for check_repeat_fn (function)
macro_rules! Depcrate_repeat_vec_with_capacitycheck_repeat_fn {
() => {
// Module: crate::repeat_vec_with_capacity
// Provides: {"check_repeat_fn"}
// Dependencies: {}
# [doc = " Checks `iter::repeat(Vec::with_capacity(x))`"] fn check_repeat_fn (cx : & LateContext < '_ > , expr : & Expr < '_ > , msrv : Msrv) { if ! expr . span . from_expansion () && fn_def_id (cx , expr) . is_some_and (| did | cx . tcx . is_diagnostic_item (sym :: iter_repeat , did)) && let ExprKind :: Call (_ , [repeat_expr]) = expr . kind && fn_def_id (cx , repeat_expr) . is_some_and (| did | cx . tcx . is_diagnostic_item (sym :: vec_with_capacity , did)) && ! repeat_expr . span . from_expansion () && let Some (exec_context) = std_or_core (cx) && msrv . meets (cx , msrvs :: REPEAT_WITH) { emit_lint (cx , expr . span , "iter::repeat" , "none of the yielded `Vec`s will have the requested capacity" , "if you intended to create an iterator that yields `Vec`s with an initial capacity, try" , format ! ("{exec_context}::iter::repeat_with(|| {})" , snippet (cx , repeat_expr . span , "..")) ,) ; } }
};
}
