// Generated macro for is_ref_char (function)
macro_rules! Depcrate_methods_single_char_add_stris_ref_char {
() => {
// Module: crate::methods::single_char_add_str
// Provides: {"is_ref_char"}
// Dependencies: {}
fn is_ref_char (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { matches ! (cx . typeck_results () . expr_ty (expr) . kind () , ty :: Ref (_ , ty , _) if ty . is_char ()) }
};
}
