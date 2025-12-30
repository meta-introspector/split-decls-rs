// Generated macro for is_integer_const (function)
macro_rules! Depcrateis_integer_const {
() => {
// Module: crate
// Provides: {"is_integer_const"}
// Dependencies: {}
# [doc = " Checks whether the given expression is a constant integer of the given value."] # [doc = " unlike `is_integer_literal`, this version does const folding"] pub fn is_integer_const (cx : & LateContext < '_ > , e : & Expr < '_ > , value : u128) -> bool { if is_integer_literal (e , value) { return true ; } let enclosing_body = cx . tcx . hir_enclosing_body_owner (e . hir_id) ; if let Some (Constant :: Int (v)) = ConstEvalCtxt :: with_env (cx . tcx , cx . typing_env () , cx . tcx . typeck (enclosing_body)) . eval (e) { return value == v ; } false }
};
}
