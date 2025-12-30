// Generated macro for expr_custom_deref_adjustment (function)
macro_rules! Depcrateexpr_custom_deref_adjustment {
() => {
// Module: crate
// Provides: {"expr_custom_deref_adjustment"}
// Dependencies: {}
# [doc = " Gets the mutability of the custom deref adjustment, if any."] pub fn expr_custom_deref_adjustment (cx : & LateContext < '_ > , e : & Expr < '_ >) -> Option < Mutability > { cx . typeck_results () . expr_adjustments (e) . iter () . find_map (| a | match a . kind { Adjust :: Deref (Some (d)) => Some (Some (d . mutbl)) , Adjust :: Deref (None) => None , _ => Some (None) , }) . and_then (| x | x) }
};
}
