// Generated macro for binops_with_local (function)
macro_rules! Depcrate_transmute_eager_transmutebinops_with_local {
() => {
// Module: crate::transmute::eager_transmute
// Provides: {"binops_with_local"}
// Dependencies: {}
# [doc = " Checks if a given expression is a binary operation involving a local variable or is made up of"] # [doc = " other (nested) binary expressions involving the local. There must be at least one local"] # [doc = " reference that is the same as `local_expr`."] # [doc = ""] # [doc = " This is used as a heuristic to detect if a variable"] # [doc = " is checked to be within the valid range of a transmuted type."] # [doc = " All of these would return true:"] # [doc = " * `x < 4`"] # [doc = " * `x < 4 && x > 1`"] # [doc = " * `x.field < 4 && x.field > 1` (given `x.field`)"] # [doc = " * `x.field < 4 && unrelated()`"] # [doc = " * `(1..=3).contains(&x)`"] fn binops_with_local (cx : & LateContext < '_ > , local_expr : & Expr < '_ > , expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Binary (_ , lhs , rhs) => { binops_with_local (cx , local_expr , lhs) || binops_with_local (cx , local_expr , rhs) } , ExprKind :: MethodCall (path , receiver , [arg] , _) if path . ident . name == sym :: contains && let Some (receiver_adt) = cx . typeck_results () . expr_ty (receiver) . peel_refs () . ty_adt_def () && let lang_items = cx . tcx . lang_items () && [lang_items . range_from_struct () , lang_items . range_inclusive_struct () , lang_items . range_struct () , lang_items . range_to_inclusive_struct () , lang_items . range_to_struct ()] . into_iter () . any (| did | did == Some (receiver_adt . did ())) => { eq_expr_value (cx , local_expr , arg . peel_borrows ()) } , _ => eq_expr_value (cx , local_expr , expr) , } }
};
}
