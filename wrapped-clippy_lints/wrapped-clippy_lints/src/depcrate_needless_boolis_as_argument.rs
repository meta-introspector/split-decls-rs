// Generated macro for is_as_argument (function)
macro_rules! Depcrate_needless_boolis_as_argument {
() => {
// Module: crate::needless_bool
// Provides: {"is_as_argument"}
// Dependencies: {}
fn is_as_argument (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { matches ! (get_parent_expr (cx , e) . map (| e | e . kind) , Some (ExprKind :: Cast (_ , _))) }
};
}
