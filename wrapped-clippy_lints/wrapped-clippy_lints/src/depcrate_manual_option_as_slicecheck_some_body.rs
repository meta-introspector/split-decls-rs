// Generated macro for check_some_body (function)
macro_rules! Depcrate_manual_option_as_slicecheck_some_body {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"check_some_body"}
// Dependencies: {}
# [doc = " Returns true if `expr` is `std::slice::from_ref(<name>)`. Used in `if let`s."] fn check_some_body (cx : & LateContext < '_ > , name : Symbol , expr : & Expr < '_ >) -> bool { if let ExprKind :: Call (slice_from_ref , [arg]) = expr . peel_blocks () . kind && is_slice_from_ref (cx , slice_from_ref) && let ExprKind :: Path (QPath :: Resolved (None , path)) = arg . kind && let [seg] = path . segments { seg . ident . name == name } else { false } }
};
}
