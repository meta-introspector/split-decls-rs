// Generated macro for check_for_either_unsigned_int_constant (function)
macro_rules! Depcrate_manual_rem_euclidcheck_for_either_unsigned_int_constant {
() => {
// Module: crate::manual_rem_euclid
// Provides: {"check_for_either_unsigned_int_constant"}
// Dependencies: {}
fn check_for_either_unsigned_int_constant < 'a > (cx : & 'a LateContext < '_ > , ctxt : SyntaxContext , left : & 'a Expr < '_ > , right : & 'a Expr < '_ > ,) -> Option < (u128 , & 'a Expr < 'a >) > { check_for_unsigned_int_constant (cx , ctxt , left) . map (| int_const | (int_const , right)) . or_else (| | check_for_unsigned_int_constant (cx , ctxt , right) . map (| int_const | (int_const , left))) }
};
}
