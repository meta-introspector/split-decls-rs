// Generated macro for check_vec_macro (function)
macro_rules! Depcrate_repeat_vec_with_capacitycheck_vec_macro {
() => {
// Module: crate::repeat_vec_with_capacity
// Provides: {"check_vec_macro"}
// Dependencies: {}
# [doc = " Checks `vec![Vec::with_capacity(x); n]`"] fn check_vec_macro (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if matching_root_macro_call (cx , expr . span , sym :: vec_macro) . is_some () && let Some (VecArgs :: Repeat (repeat_expr , len_expr)) = VecArgs :: hir (cx , expr) && fn_def_id (cx , repeat_expr) . is_some_and (| did | cx . tcx . is_diagnostic_item (sym :: vec_with_capacity , did)) && ! len_expr . span . from_expansion () && let Some (Constant :: Int (2 ..)) = ConstEvalCtxt :: new (cx) . eval (expr_or_init (cx , len_expr)) { emit_lint (cx , expr . span . source_callsite () , "vec![x; n]" , "only the last `Vec` will have the capacity" , "if you intended to initialize multiple `Vec`s with an initial capacity, try" , format ! ("(0..{}).map(|_| {}).collect::<Vec<_>>()" , snippet (cx , len_expr . span , "") , snippet (cx , repeat_expr . span , "..")) ,) ; } }
};
}
