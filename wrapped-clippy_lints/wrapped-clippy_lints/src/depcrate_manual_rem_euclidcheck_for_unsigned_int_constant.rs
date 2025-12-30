// Generated macro for check_for_unsigned_int_constant (function)
macro_rules! Depcrate_manual_rem_euclidcheck_for_unsigned_int_constant {
() => {
// Module: crate::manual_rem_euclid
// Provides: {"check_for_unsigned_int_constant"}
// Dependencies: {}
fn check_for_unsigned_int_constant < 'a > (cx : & 'a LateContext < '_ > , ctxt : SyntaxContext , expr : & 'a Expr < '_ > ,) -> Option < u128 > { let int_const = ConstEvalCtxt :: new (cx) . eval_full_int (expr , ctxt) ? ; match int_const { FullInt :: S (s) => s . try_into () . ok () , FullInt :: U (u) => Some (u) , } }
};
}
