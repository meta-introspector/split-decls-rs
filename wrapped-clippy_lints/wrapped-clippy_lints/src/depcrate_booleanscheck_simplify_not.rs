// Generated macro for check_simplify_not (function)
macro_rules! Depcrate_booleanscheck_simplify_not {
() => {
// Module: crate::booleans
// Provides: {"check_simplify_not"}
// Dependencies: {}
fn check_simplify_not (cx : & LateContext < '_ > , msrv : Msrv , expr : & Expr < '_ >) { if let ExprKind :: Unary (UnOp :: Not , inner) = & expr . kind && ! expr . span . from_expansion () && ! inner . span . from_expansion () && let Some (suggestion) = simplify_not (cx , msrv , inner) && cx . tcx . lint_level_at_node (NONMINIMAL_BOOL , expr . hir_id) . level != Level :: Allow { use clippy_utils :: sugg :: { Sugg , has_enclosing_paren } ; let maybe_par = if let Some (sug) = Sugg :: hir_opt (cx , inner) { match sug { Sugg :: BinOp (..) => true , Sugg :: MaybeParen (sug) if ! has_enclosing_paren (& sug) => true , _ => false , } } else { false } ; let suggestion = if maybe_par { format ! ("({suggestion})") } else { suggestion } ; span_lint_and_sugg (cx , NONMINIMAL_BOOL , expr . span , "this boolean expression can be simplified" , "try" , suggestion , Applicability :: MachineApplicable ,) ; } }
};
}
